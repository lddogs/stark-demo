from merkle import Merkle
from os import urandom

def test_merkle():
    n = 64
    leafs = [urandom(int(urandom(1)[0])) for i in range(n)]
    root = Merkle.commit_(leafs)

    # mở bất kỳ lá nào cũng nên hoạt động
    for i in range(n):
        path = Merkle.open_(i, leafs)
        assert(Merkle.verify_(root, i, path, leafs[i]))

    # mở các nút không phải lá sẽ không hoạt động
    for i in range(n):
        path = Merkle.open_(i, leafs)
        assert(False == Merkle.verify_(root, i, path, urandom(51)))

    # mở các lá sai sẽ không hoạt động
    for i in range(n):
        path = Merkle.open_(i, leafs)
        j = (i + 1 + (int(urandom(1)[0] % (n-1)))) % n
        assert(False == Merkle.verify_(root, i, path, leafs[j]))

    # mở các lá với chỉ số sai sẽ không hoạt động
    for i in range(n):
        path = Merkle.open_(i, leafs)
        j = (i + 1 + (int(urandom(1)[0] % (n-1)))) % n
        assert(False == Merkle.verify_(root, j, path, leafs[i]))

    # mở các lá đến một gốc sai sẽ không hoạt động
    for i in range(n):
        path = Merkle.open_(i, leafs)
        assert(False == Merkle.verify_(urandom(32), i, path, leafs[i]))

    # mở các lá với ít nhất một sai sót trong đường dẫn sẽ không hoạt động
    for i in range(n):
        path = Merkle.open_(i, leafs)
        for j in range(len(path)):
            fake_path = path[0:j] + [urandom(32)] + path[j+1:]
            assert(False == Merkle.verify_(root, i, fake_path, leafs[i]))

    # mở các lá đến một gốc khác sẽ không hoạt động
    fake_root = Merkle.commit_([urandom(32) for i in range(n)])
    for i in range(n):
        path = Merkle.open_(i, leafs)
        assert(False == Merkle.verify_(fake_root, i, path, leafs[i]))

test_merkle()
