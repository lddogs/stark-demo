# Luồng chứng minh và Fiat-Shamir

File `ip.py` định nghĩa lớp `ProofStream`, một cấu trúc dữ liệu quan trọng được sử dụng trong các giao thức chứng minh tương tác và không tương tác. Lớp này cung cấp các phương thức để quản lý luồng chứng minh và thực hiện biến đổi Fiat-Shamir.

## Lớp ProofStream

### Khởi tạo

```python
def __init__( self ):
    self.objects = []
    self.read_index = 0
```

Khởi tạo một `ProofStream` mới với một danh sách đối tượng rỗng và chỉ số đọc bằng 0.

### Phương thức push

```python
def push( self, obj ):
    self.objects += [obj]
```

Thêm một đối tượng mới vào cuối danh sách.

### Phương thức pull

```python
def pull( self ):
    assert(self.read_index < len(self.objects)), "ProofStream: cannot pull object; queue empty."
    obj = self.objects[self.read_index]
    self.read_index += 1
    return obj
```

Lấy đối tượng tiếp theo từ danh sách và tăng chỉ số đọc. Nếu không còn đối tượng nào, phương thức sẽ báo lỗi.

### Phương thức serialize và deserialize

```python
def serialize( self ):
    return pickle.dumps(self.objects)

@staticmethod
def deserialize(bb):
    ps = ProofStream()
    ps.objects = pickle.loads(bb)
    return ps
```

Các phương thức này cho phép chuyển đổi `ProofStream` thành dạng byte và ngược lại, sử dụng thư viện `pickle` để tuần tự hóa.

### Phương thức Fiat-Shamir

```python
def prover_fiat_shamir( self, num_bytes=32 ):
    return shake_256(self.serialize()).digest(num_bytes)

def verifier_fiat_shamir( self, num_bytes=32 ):
    return shake_256(pickle.dumps(self.objects[:self.read_index])).digest(num_bytes)
```

Hai phương thức này thực hiện biến đổi Fiat-Shamir, một kỹ thuật quan trọng để chuyển đổi giao thức chứng minh tương tác thành không tương tác:

- `prover_fiat_shamir`: Tính toán giá trị băm của toàn bộ luồng chứng minh.
- `verifier_fiat_shamir`: Tính toán giá trị băm của phần luồng chứng minh đã được đọc.

Cả hai phương thức đều sử dụng hàm băm SHAKE-256 với độ dài đầu ra mặc định là 32 byte. Khác nhau ở việc `prover_fiat_shamir` băm toàn bộ luồng chứng minh hay còn `verifier_fiat_shamir` băm phần đã được đọc.

## Ý nghĩa và ứng dụng

`ProofStream` đóng vai trò quan trọng trong việc xây dựng và xác minh các chứng minh zero-knowledge. Nó cho phép:

1. Lưu trữ và quản lý các bước của quá trình chứng minh.
2. Thực hiện biến đổi Fiat-Shamir để tạo ra các thách thức ngẫu nhiên trong giao thức không tương tác.
3. Tuần tự hóa và giải tuần tự hóa luồng chứng minh để truyền qua mạng hoặc lưu trữ.

Việc sử dụng `ProofStream` giúp đơn giản hóa quá trình triển khai các giao thức chứng minh phức tạp, đồng thời đảm bảo tính nhất quán giữa người chứng minh và người xác minh trong quá trình tạo và kiểm tra chứng minh.

