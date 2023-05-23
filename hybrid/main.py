import os
import json
import argparse
import numpy as np
from e5 import e5_embeddings
from datasets import load_dataset

parser = argparse.ArgumentParser()
parser.add_argument('--model', type=str, default='combination', help='neural, lexical or combination model')
parser.add_argument('-k', type=int, default=60, help='k for RRF')
args = parser.parse_args()
k = args.k
model = args.model

data = load_dataset("ms_marco", "v2.1", split='validation[:1%]')
data = data.filter(lambda x: sum(x['passages']['is_selected']) == 1)
passages = data['passages']

R = [x['is_selected'].index(1) for x in passages]
D = [x['passage_text'] for x in passages]
D_flat = [d for doc in D for d in doc]
Q = data['query']
QID = data['query_id']

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
    N = len(D[i])
    avgdl = sum([len(d.split()) for d in D[i]]) / N
    scores = [rrf(i, j, N, avgdl) for j in range(N)]

    # sort by score
    scores = sorted(zip(scores, range(N), D[i]))

    # compute reciprocal rank, save results to file
    rr = 0
    for j, (_score, k, _d) in enumerate(scores):
      if rr == 0 and k == R[i]:
        rr = 1 / (j + 1)
        break
    mrr += rr

  print ('MRR: ' + str(mrr / len(Q)))

# combine the two models using RRF
def rrf(i, j, N, avgdl):
  lexical = bm25(i, j, N, avgdl)

  q_emb = Q_emb[i]
  d_emb = D_emb[i][j]
  neural = np.dot(q_emb, d_emb) / (np.linalg.norm(q_emb) * np.linalg.norm(d_emb))

  if model == "neural":
    return 1 / (k + neural)
  elif model == "lexical":
    return 1 / (k + lexical)
  else:
    return 1 / (k + lexical) + 1 / (k + neural)


# lexical model is BM25 (should maybe just use Terrier)
b = 0.5
k1 = 1.2
def bm25(i, j, N, avgdl):
  q = Q[i].split()
  d = D[i][j].split()
  acc = 0
  for k in range(len(q)):
    f_k = f(q[k], d)
    acc += (k1 + 1) * f_k / (k1 * (1 - b + b * len(d) / avgdl) + f_k) * idf(q[k], N)
  return acc


def f(q_i, d):
  return sum([1 for w in d if w == q_i])


def idf(i, N):
  return np.log((N - n(i) + 0.5) / (n(i) + 0.5))


n_memo = {}
def n(q_i):
  if q_i in n_memo:
    return n_memo[q_i]
  ret = sum([1 for d in D if q_i in d])
  n_memo[q_i] = ret
  return ret


if __name__ == '__main__':
  test()
