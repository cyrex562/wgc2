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
            self.peers = []

class Key:
    def __init__(self, key: str = ""):
        self.key = key