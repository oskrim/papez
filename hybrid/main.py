import os
import json
import argparse
from collections import Counter
import numpy as np
np.random.seed(42)

from e5 import e5_embeddings
from datasets import load_dataset
from pinecone.bm25_encode import BM25Encoder
from pinecone.bm25_token import BM25Tokenizer

parser = argparse.ArgumentParser()
parser.add_argument('--model', type=str, default='both', help='neural, lexical or combination model')
parser.add_argument('--bm25_model', type=str, default='python', help='python or pinecone')
parser.add_argument('-k', type=int, default=60, help='k for RRF')
parser.add_argument('--fraction', type=float, default=0.01, help='fraction of data to use')
args = parser.parse_args()
k = args.k
model = args.model
percent = int(args.fraction * 100)
bm25_model = args.bm25_model

data = load_dataset("ms_marco", "v2.1", split=f"validation[:{percent}%]")
data = data.filter(lambda x: sum(x['passages']['is_selected']) == 1)
passages = data['passages']

R = [x['is_selected'].index(1) for x in passages]
D = [x['passage_text'] for x in passages]
D_flat = [d for doc in D for d in doc]
Q = data['query']
QID = data['query_id']

b = 0.5
k1 = 1.2
N = len(D_flat)

doc_freq = None
avgdl = None
pinecone_bm25 = None
pincone_queries = None
pinecone_tokenizer = None
if bm25_model == "pinecone":
  pinecone_bm25 = BM25Encoder(b=b, k1=k1)
  pinecone_bm25.fit(D_flat)
  pinecone_queries = pinecone_bm25.encode_queries(Q)
else:
  pinecone_tokenizer = BM25Tokenizer()
  D_flat = [pinecone_tokenizer(d) for d in D_flat]
  avgdl = sum([len(d) for d in D_flat]) / N
  doc_freq_counter = Counter()
  for d in D_flat:
    doc_freq_counter.update(d)
  doc_freq = dict(doc_freq_counter)

print(f"Using {percent}% of the data, dataset size: {len(Q)}")

Q_emb = []
D_emb = []
if os.getenv("COMPUTE"):
  for i in range(len(Q)):
    print(i)
    D_emb.append(e5_embeddings([f"passage: #{x}" for x in D[i]]))
    Q_emb.extend(e5_embeddings([f"query: #{x}" for x in [Q[i]]]))

  json.dump(D_emb, open('D_emb.json', 'w'))
  json.dump(Q_emb, open('Q_emb.json', 'w'))
else:
  D_emb = json.load(open('D_emb.json'))
  Q_emb = json.load(open('Q_emb.json'))

# test the model
def test():
  mrr = 0
  answers = []
  for i in range(len(Q)):
    # sort documents by score
    scores = [rrf(i, j) for j in range(len(D[i]))]
    scores = sorted(zip(scores, range(len(D[i]))))

    # compute reciprocal rank, save results to file
    rr = 0
    for j, (_score, k) in enumerate(scores):
      if rr == 0 and k == R[i]:
        rr = 1 / (j + 1)
        break
    mrr += rr

  print ('MRR: ' + str(mrr / len(Q)))

# combine the two models using RRF
def rrf(*args):
  if model == "random":
    return 1 / (k + np.random.rand())
  elif model == "neural":
    return 1 / (k + neural(*args))
  elif model == "lexical":
    return 1 / (k + lexical(*args))
  else:
    lex = lexical(*args)
    neur = neural(*args)
    return 1 / (k + lex) + 1 / (k + neur)


def neural(i, j):
  q_emb = Q_emb[i]
  d_emb = D_emb[i][j]
  return np.dot(q_emb, d_emb) / (np.linalg.norm(q_emb) * np.linalg.norm(d_emb))


def lexical(i, j):
  if bm25_model == "pinecone":
    return sparse_dot(pinecone_bm25.encode_documents(D[i][j]), pinecone_queries[i])
  return bm25(i, j)


def sparse_dot(u, v):
  u_ptr = get_pointers(u)
  v_ptr = get_pointers(v)
  dot = sum([u_ptr[i] * v_ptr[i] for i in u_ptr if i in v_ptr])
  return dot / (np.linalg.norm(u["values"]) * np.linalg.norm(v["values"]))


def get_pointers(v):
  return {v["indices"][i]: v["values"][i] for i in range(len(v["indices"]))}


def bm25(i, j):
  q = pinecone_tokenizer(Q[i])
  d = pinecone_tokenizer(D[i][j])
  acc = 0
  idf_sum = 0
  for k in range(len(q)):
    f_k = f(q[k], d)
    idf_factor = idf(q[k])
    idf_sum += idf_factor
    acc += (k1 + 1) * f_k / (k1 * (1 - b + b * len(d) / avgdl) + f_k) * idf_factor
  return acc / idf_sum


def f(q_i, d):
  return sum([1 for w in d if w == q_i])


def idf(i):
  # return np.log((N - n(i) + 0.5) / (n(i) + 0.5))
  tf = doc_freq.get(i, 1)
  # return np.log((N - tf + 0.5) / (tf + 0.5))
  return np.log((N + 1) / (tf + 0.5))


n_memo = {}
def n(q_i):
  if q_i in n_memo:
    return n_memo[q_i]
  ret = sum([1 for d in D if q_i in d])
  n_memo[q_i] = ret
  return ret


def normalized_dot(u, v):
  return np.dot(u, v) / (np.linalg.norm(u) * np.linalg.norm(v))


if __name__ == '__main__':
  test()
