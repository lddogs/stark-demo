# FRI (Fast Reed-Solomon Interactive Oracle Proof)

File `fri.py` định nghĩa lớp `Fri`, cung cấp một triển khai của giao thức FRI (Fast Reed-Solomon Interactive Oracle Proof). FRI là một giao thức chứng minh không tương tác được sử dụng để chứng minh rằng một đa thức có bậc thấp.

## Lớp Fri

### Khởi tạo

```python
def __init__( self, offset, omega, initial_domain_length, expansion_factor, num_colinearity_tests ):
    self.offset = offset
    self.omega = omega
    self.domain_length = initial_domain_length
    self.field = omega.field
    self.expansion_factor = expansion_factor
    self.num_colinearity_tests = num_colinearity_tests
```

Khởi tạo một đối tượng FRI với các tham số cần thiết.

1. `self.offset`:
   - Ý nghĩa: Đây là giá trị offset được sử dụng để xác định miền đánh giá của đa thức.
   - Công thức: Miền đánh giá thường được định nghĩa là $D = \{offset \cdot \omega^i \mid i = 0, 1, ..., domain\_length - 1\}$

2. `self.omega`:
   - Ý nghĩa: Đây là phần tử nguyên thủy thứ n của trường hữu hạn, trong đó n là độ dài miền ban đầu.
   - Công thức: $\omega^n = 1$ và $\omega^k \neq 1$ với mọi $k < n$

3. `self.domain_length`:
   - Ý nghĩa: Đây là kích thước của miền đánh giá ban đầu.
   - Công thức: $|D| = domain\_length$

4. `self.field`:
   - Ý nghĩa: Đây là trường hữu hạn mà các phép tính được thực hiện trên đó.

5. `self.expansion_factor`:
   - Ý nghĩa: Đây là hệ số mở rộng sử dụng trong giao thức FRI. Nó xác định mức độ dư thừa của mã Reed-Solomon.
   - Công thức: Tỷ lệ giữa độ dài codeword và bậc đa thức là $expansion\_factor$.

6. `self.num_colinearity_tests`:
   - Ý nghĩa: Đây là số lượng kiểm tra đồng tuyến tính được thực hiện trong mỗi vòng của giao thức FRI.
   - Liên quan đến xác suất lỗi: Xác suất lỗi giảm theo hàm mũ với số lượng kiểm tra, có thể biểu diễn gần đúng là $O((1/|F|)^{num\_colinearity\_tests})$, trong đó $|F|$ là kích thước của trường.

Các tham số này cùng nhau xác định cấu trúc và độ bảo mật của giao thức FRI:
- `offset` và `omega` xác định miền đánh giá $D$.
- `domain_length` xác định kích thước của bài toán, $n = domain\_length$.
- `expansion_factor` ảnh hưởng đến độ dài của bằng chứng và độ bảo mật. Nếu $d$ là bậc của đa thức, thì $n = expansion\_factor \cdot d$.
- `num_colinearity_tests` ảnh hưởng trực tiếp đến độ bảo mật của giao thức, với xác suất lỗi khoảng $\epsilon \approx (d/|F|)^{num\_colinearity\_tests}$.

Các tham số này cần được chọn cẩn thận để cân bằng giữa hiệu suất và độ bảo mật của giao thức FRI. Ví dụ, tăng `expansion_factor` và `num_colinearity_tests` sẽ cải thiện độ bảo mật nhưng cũng làm tăng chi phí tính toán và kích thước bằng chứng.

### Phương thức num_rounds

```python
def num_rounds( self ):
    codeword_length = self.domain_length
    num_rounds = 0
    while codeword_length > self.expansion_factor and 4*self.num_colinearity_tests < codeword_length:
        codeword_length /= 2
        num_rounds += 1
    return num_rounds
```

Tính số vòng FRI dựa trên độ dài miền ban đầu và các tham số khác.

### Phương thức sample_indices

```python
def sample_indices( self, seed, size, reduced_size, number ):
    # ... (code implementation)
```

Lấy mẫu ngẫu nhiên các chỉ số để kiểm tra trong giao thức FRI.

### Phương thức commit

```python
def commit( self, codeword, proof_stream, round_index=0 ):
    # ... (code implementation)
```

Thực hiện giai đoạn commit của giao thức FRI, tạo ra các cam kết Merkle và thực hiện quá trình gấp đôi.

### Phương thức query

```python
def query( self, current_codeword, next_codeword, c_indices, proof_stream ):
    # ... (code implementation)
```

Thực hiện giai đoạn truy vấn của giao thức FRI, tiết lộ các giá trị cần thiết và đường dẫn xác thực Merkle.

### Phương thức prove

```python
def prove( self, codeword, proof_stream ):
    # ... (code implementation)
```

Tạo ra một bằng chứng FRI đầy đủ cho một codeword đã cho.

### Phương thức verify

```python
def verify( self, proof_stream, polynomial_values ):
    # ... (code implementation)
```

Xác minh một bằng chứng FRI đã cho.

## Ý nghĩa và ứng dụng

Giao thức FRI là một thành phần quan trọng trong nhiều hệ thống chứng minh zero-knowledge, đặc biệt là trong các hệ thống dựa trên STARK (Scalable Transparent ARgument of Knowledge). Nó cho phép:

1. Chứng minh hiệu quả rằng một đa thức có bậc thấp mà không tiết lộ toàn bộ đa thức.
2. Giảm đáng kể kích thước của bằng chứng so với các phương pháp truyền thống.
3. Đạt được hiệu suất cao trong việc tạo và xác minh bằng chứng.

FRI được sử dụng rộng rãi trong các ứng dụng blockchain và các hệ thống yêu cầu tính minh bạch và khả năng mở rộng cao. Nó là nền tảng cho các giao thức chứng minh phức tạp hơn như STARK, cho phép xác minh các tính toán tùy ý một cách hiệu quả và có thể mở rộng.

## Test FRI

Phần này khó hiểu hơn mấy phần trước nên để nắm chắc hơn cần xem file `test_fri.py`

Tôi sẽ giải thích các bước chính trong file test_fri.py bằng công thức toán học:

1. Khởi tạo các tham số:

```python
field = Field.main()
degree = 63
expansion_factor = 4
num_colinearity_tests = 17

initial_codeword_length = (degree + 1) * expansion_factor
```
   - Trường hữu hạn $\mathbb{F}$
   - Bậc đa thức $d = 63$
   - Hệ số mở rộng $k = 4$
   - Số lượng kiểm tra đồng tuyến tính $n = 17$

2. Tính độ dài từ mã ban đầu:
   $L = (d+1) \cdot k = 64 \cdot 4 = 256$

3. Tính căn nguyên thủy bậc $L$ của trường:

```python
omega = field.primitive_nth_root(initial_codeword_length)
```

   $\omega \in \mathbb{F}$ sao cho $\omega^L = 1$ và $\omega^{L/2} \neq 1$

4. Khởi tạo đa thức kiểm tra:

```python
coeffs = [field(i) for i in range(degree + 1)]
p = Polynomial(coeffs)
```

   $p(x) = \sum_{i=0}^d i \cdot x^i$

5. Tính từ mã:

```python
domain = [field.mul(offset, omega.pow(i)) for i in range(initial_codeword_length)]
codeword = [p.evaluate(x) for x in domain]
```

   $c = (p(\omega^0), p(\omega^1), \ldots, p(\omega^{L-1}))$

6. Thực hiện quá trình chứng minh FRI:

```python
proof_stream = ProofStream()
fri.prove(codeword, proof_stream)
```

   - Commit các lớp từ mã
   - Lấy mẫu các chỉ số ngẫu nhiên
   - Thực hiện các truy vấn

7. Xác minh bằng chứng:

```python
assert fri.verify(proof_stream, codeword[:2])
```

   - Kiểm tra tính nhất quán giữa các lớp
   - Kiểm tra tính đồng tuyến tính
   - Xác minh đường dẫn xác thực Merkle

8. Kiểm tra từ mã không hợp lệ:

```python
invalid_codeword = codeword.copy()
invalid_codeword[0] += field(1)
proof_stream = ProofStream()
fri.prove(invalid_codeword, proof_stream)
assert not fri.verify(proof_stream, invalid_codeword[:2])
```

   - Sửa đổi một phần của từ mã
   - Thực hiện quá trình chứng minh
   - Xác minh rằng bằng chứng bị từ chối

Các bước chính trong quá trình FRI:

1. Commit: $c_i \rightarrow c_{i+1}$ bằng cách:
   $c_{i+1}[j] = \frac{1}{2}((1 + \frac{\alpha}{\gamma \omega^j})c_i[j] + (1 - \frac{\alpha}{\gamma \omega^j})c_i[j+L/2])$

   Trong đó $\alpha$ là thách thức ngẫu nhiên, $\gamma$ là offset, $\omega$ là căn nguyên thủy.

2. Kiểm tra đồng tuyến tính:
   Cho 3 điểm $(x_1, y_1)$, $(x_2, y_2)$, $(x_3, y_3)$, kiểm tra:
   
   $\frac{y_2 - y_1}{x_2 - x_1} = \frac{y_3 - y_1}{x_3 - x_1}$

3. Xác minh Merkle:
   Kiểm tra xem giá trị lá và đường dẫn xác thực có khớp với gốc Merkle đã commit hay không.

Quá trình này đảm bảo rằng từ mã ban đầu tương ứng với một đa thức bậc thấp, mà không cần tiết lộ toàn bộ đa thức hoặc từ mã.
