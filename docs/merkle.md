# Cây Merkle và các Hoạt động

File `merkle.py` định nghĩa lớp `Merkle`, cung cấp các phương thức để xây dựng, mở và xác minh cây Merkle. Cây Merkle là một cấu trúc dữ liệu quan trọng trong mật mã học và blockchain, cho phép xác minh hiệu quả sự tồn tại của một phần tử trong một tập dữ liệu lớn.

## Lớp Merkle

### Hàm băm

```python
H = blake2b
```

Lớp Merkle sử dụng hàm băm BLAKE2b làm hàm băm mặc định.

### Phương thức commit_

```python
def commit_( leafs ):
    assert(len(leafs) & (len(leafs)-1) == 0), "length must be power of two"
    if len(leafs) == 1:
        return leafs[0]
    else:
        return Merkle.H(Merkle.commit_(leafs[:len(leafs)//2]) + Merkle.commit_(leafs[len(leafs)//2:])).digest()
```

Phương thức này tạo ra root của cây Merkle từ một danh sách các lá. Nó hoạt động đệ quy, chia danh sách thành hai nửa và kết hợp các giá trị băm cho đến khi đạt được root.

### Phương thức commit

```python
def commit( data_array ):
    return Merkle.commit_([Merkle.H(bytes(da)).digest() for da in data_array])
```

Phương thức này tạo ra root của cây Merkle từ một mảng dữ liệu bằng cách băm từng phần tử dữ liệu trước khi gọi `commit_`.

### Phương thức open_

```python
def open_( index, leafs ):
    assert(len(leafs) & (len(leafs)-1) == 0), "length must be power of two"
    assert(0 <= index and index < len(leafs)), "cannot open invalid index"
    if len(leafs) == 2:
        return [leafs[1 - index]]
    elif index < (len(leafs)/2):
        return Merkle.open_(index, leafs[:len(leafs)//2]) + [Merkle.commit_(leafs[len(leafs)//2:])]
    else:
        return Merkle.open_(index - len(leafs)//2, leafs[len(leafs)//2:]) + [Merkle.commit_(leafs[:len(leafs)//2])]
```

Phương thức này tạo ra một đường dẫn Merkle cho một lá cụ thể trong cây. Đường dẫn này chứa các nút cần thiết để xác minh sự tồn tại của lá trong cây.

### Phương thức open

```python
def open( index, data_array ):
    return Merkle.open_(index, [Merkle.H(bytes(da)).digest() for da in data_array])
```

Phương thức này tạo ra một đường dẫn Merkle cho một phần tử dữ liệu cụ thể trong mảng dữ liệu.

### Phương thức verify_

```python
def verify_( root, index, path, leaf ):
    assert(0 <= index and index < (1 << len(path))), "cannot verify invalid index"
    if len(path) == 1:
        if index == 0:
            return root == Merkle.H(leaf + path[0]).digest()
        else:
            return root == Merkle.H(path[0] + leaf).digest()
    else:
        if index % 2 == 0:
            return Merkle.verify_(root, index >> 1, path[1:], Merkle.H(leaf + path[0]).digest())
        else:
            return Merkle.verify_(root, index >> 1, path[1:], Merkle.H(path[0] + leaf).digest())
```

Phương thức này xác minh xem một lá cụ thể có tồn tại trong cây Merkle hay không, sử dụng root, index, đường dẫn Merkle và giá trị của lá.

### Phương thức verify

```python
def verify( root, index, path, data_element ):
    return Merkle.verify_(root, index, path, Merkle.H(bytes(data_element)).digest())
```

Phương thức này xác minh sự tồn tại của một phần tử dữ liệu trong cây Merkle.

## Ý nghĩa và ứng dụng

Cây Merkle có nhiều ứng dụng quan trọng trong mật mã học và blockchain:

1. Xác minh hiệu quả: Cho phép xác minh sự tồn tại của một phần tử trong một tập dữ liệu lớn mà không cần truy cập toàn bộ tập dữ liệu.
2. Tính toàn vẹn dữ liệu: Bất kỳ thay đổi nào trong dữ liệu sẽ dẫn đến thay đổi trong root của cây Merkle.
3. Đồng bộ hóa hiệu quả: Cho phép xác định nhanh chóng sự khác biệt giữa các tập dữ liệu lớn.
4. Chứng minh không tiết lộ: Có thể chứng minh sự tồn tại của một phần tử mà không tiết lộ toàn bộ tập dữ liệu.

Việc sử dụng cây Merkle trong các ứng dụng blockchain và mật mã học giúp tăng cường bảo mật, hiệu quả và khả năng mở rộng của các hệ thống này.

