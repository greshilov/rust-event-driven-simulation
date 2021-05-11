import hmac
import hashlib
import struct
import sys
import requests


def make_signed(player_name: str, player_uuid: str, score: int, ticks_per_sec: int, secret: str):
    """
    Current secret (2021-05-11) is "library/app/dist/"
    """
    return dict(
        game_result=dict(
            player_name=player_name,
            player_uuid=player_uuid,
            score=score,
            ticks_per_sec=ticks_per_sec,
        ),
        hex_digest=list(hmac.new(
            secret.encode(),
            player_name.encode() + player_uuid.encode() + struct.pack('>I', score) + struct.pack('>I', ticks_per_sec),
            hashlib.sha256,
        ).digest())
    )


def main(argv):
    requests.post(
        'https://b.greshilov.me/reds/api/submit',
        json=make_signed(argv[1], argv[2], int(argv[3]), int(argv[4]), argv[5]),
    )


if __name__ == '__main__':
    main(sys.argv)
