from typing import List


class Peer:
    def __init__(self, private_key: str = "",
                 public_key: str = "",
                 endpoint: str = "",
                 persistent_keepalive=25,
                 allowed_ips=""):
        self.private_key = private_key
        self.public_key = public_key
        self.endpoint = endpoint
        self.persistent_keepalive = persistent_keepalive
        self.allowed_ips = allowed_ips

    def __str__(self):
        return f"private_key={self.private_key}, public_key={self.public_key}, endpoint={self.endpoint}, persistent_keepalive={self.persistent_keepalive}, allowed_ips={self.allowed_ips}"


class Interface:
    def __init__(self,
                 name: str = "",
                 public_key: str = "",
                 private_key: str = "",
                 listen_port: int = 0,
                 address: str = "",
                 peers: List = None):
        self.name = name
        self.public_key = public_key
        self.private_key = private_key
        self.listen_port = listen_port
        self.address = address
        if peers is None:
            peers = []
        self.peers = peers

    def __str__(self):
        return f"name={self.name}, public_key={self.public_key}, private_key={self.private_key}, listen_port={self.listen_port}, address={self.address}, peers={self.peers}"


class Key:
    def __init__(self, key: str = ""):
        self.key = key
