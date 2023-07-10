import sys
import socket
import struct
import random

ETH_P_ALL=3
bufsize = 2000
interface = 'wlan0'
ip_id = 1
seq = 0
ack = 0

s = socket.socket(socket.AF_PACKET, socket.SOCK_RAW, socket.htons(ETH_P_ALL))
s.bind((interface, 0))


def get_checksum(data):
    # if the total length is odd, pad with zeros
    if len(data) & 1:
        data += b'\x00'

    x = 0
    for i in range(len(data) // 2):
        x += struct.unpack("!H", data[i*2:i*2+2])[0]
    x = (x >> 16) + (x & 0xffff)
    x = x + (x >> 16)
    return  ~x & 0xffff


def send_tcp(r, data, flags):
    global ip_id, interface, seq, ack

    dst_mac = r[0:6]
    src_mac = r[6:12]
    eth_type = 0x0800
    eth_header = struct.pack('!6s6sH', src_mac, dst_mac, eth_type) 

    dst_port = struct.unpack("!H", r[34:36])[0]
    src_port = 8080
    offset = 5 << 4
    window = bufsize
    checksum = 0
    urgent_pointer = 0
    tcp_header = struct.pack('!HHLLBBHHH', src_port, dst_port, seq, ack, offset, flags, window, checksum, urgent_pointer)

    ip = r[14:]
    src_ip = ip[12:16]
    dst_ip = ip[16:20]
    protocol = 6
    tcp_length = len(tcp_header) + len(data)
    pseudo_header = src_ip + dst_ip + struct.pack('!BBH', 0, protocol, tcp_length)
    checksum = get_checksum(pseudo_header + tcp_header + data)
    tcp_header = struct.pack('!HHLLBBHHH', src_port, dst_port, seq, ack, offset, flags, window, checksum, urgent_pointer)

    # 4510 0080 48eb 4000 40 06 6b60 c0a80298 c0a80234
    ip_header = bytearray(ip[:5*4])
    ip_header[2:4]   = struct.pack('!H', len(ip_header) + tcp_length)
    ip_header[4:6]   = struct.pack('!H', ip_id & 0xffff)
    ip_header[10:12] = struct.pack('!H', 0)
    ip_header[12:16] = dst_ip
    ip_header[16:20] = src_ip
    ip_id += 1

    ip_header[10:12] = struct.pack('!H', get_checksum(ip_header))

    packet = eth_header + ip_header + tcp_header + data
    s.sendto(packet, (interface, 0))


def send_ack(r):
    send_tcp(r, bytes(), 0x1 << 4)


def send_synack(r):
    global seq
    seq = random.randint(0, 65535) 
    send_tcp(r, bytes(), 0x2 | (0x1 << 4))
    seq += 1


def send_fin(r):
    global seq
    send_tcp(r, bytes(), 0x1)
    seq += 1


def send_http_response(r):
    global seq
    data = b"HTTP/1.1 200 OK\r\nServer: MySimpleServer/1.0\r\nContent-Type: text/plain\r\nConnection: close\r\nContent-Length: 13\r\n\r\nHello World\r\n"
    send_tcp(r, data, (0x1 << 3) | (0x1 << 4) | 0x1)
    seq += len(data)


def test_checksum(tcp, ip):
    tcp2 = bytearray(tcp)
    checksum = struct.unpack('!H', tcp2[16:18])[0]
    tcp2[16] = 0
    tcp2[17] = 0
    src_ip = ip[12:16]
    dst_ip = ip[16:20]
    protocol = 6
    tcp_length = len(tcp)
    pseudo_header = src_ip + dst_ip + struct.pack('!BBH', 0, protocol, tcp_length)
    checksum2 = get_checksum(pseudo_header + tcp2)
    if checksum != checksum2:
        raise ValueError(f"checksum mismatch {checksum} != {checksum2}")


def parse_tcp(tcp, r):
    global ack

    src_port = struct.unpack("!H", tcp[0:2])[0]
    dst_port = struct.unpack("!H", tcp[2:4])[0]

    if dst_port != 8080:
        # print(f"port {dst_port} != 8080")
        return

    test_checksum(tcp, r[14:])
    
    seq_num = struct.unpack("!L", tcp[4:8])[0]
    ack_num = struct.unpack("!L", tcp[8:12])[0]
    offset = (tcp[12] >> 4) & 0xF

    fin = (tcp[13] >> 0) & 0x1
    syn = (tcp[13] >> 1) & 0x1
    rst = (tcp[13] >> 2) & 0x1
    psh = (tcp[13] >> 3) & 0x1
    fack = (tcp[13] >> 4) & 0x1

    print(f"src_port {src_port} dst_port {dst_port} offset {offset}")
    print(f"seq_num {seq_num} ack_num {ack_num}")
    print(f"fin {fin} syn {syn} rst {rst} psh {psh} ack {fack}")

    if syn:
        ack = seq_num + 1
        send_synack(r)
    elif fin:
        ack = seq_num + 1
        send_ack(r)
        # send_fin(r)
    else:
        data = tcp[offset*4:]
        if len(data):
            print(f"tcp data {data}")
            ack = seq_num + len(data)
            send_ack(r)
            send_http_response(r)
        else:
            print("no tcp data")

    print()


def parse_ip(ip, r):
    ihl = ip[0] & 0xF
    version = (ip[0] >> 4) & 0xF
    length = struct.unpack("!H", ip[2:4])[0]
    protocol = ip[9]
    payload = ip[ihl*4:]

    # print(f"version {version} ihl {ihl} length {length} protocol {protocol}")

    if protocol != 6:
        # print(f"want tcp(6), was {protocol}, skipping")
        return

    if length != len(ip):
        raise ValueError(f"length does not match, {length} != {len(ip)}")

    if ihl != 5:
        raise ValueError(f"assuming ihl is 5, it was {ihl}")

    return parse_tcp(payload, r)


def parse_eth(r):
    dst = r[0:6].hex()
    src = r[6:12].hex()
    ethertype = r[12:14].hex()
    ip = r[14:]

    # print(f"len {len(r)} dst {dst} src {src} ethertype {ethertype}")

    return parse_ip(ip, r)

while True:
    r = s.recv(bufsize)
    if parse_eth(r) is True:
        break
