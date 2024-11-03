# Đại số

## Thuật toán Euclid mở rộng
```python
# algebra.py
def xgcd( x, y ):
    old_r, r = (x, y)
    old_s, s = (1, 0)
    old_t, t = (0, 1)

    while r != 0:
        quotient = old_r // r
        old_r, r = (r, old_r - quotient * r)
        old_s, s = (s, old_s - quotient * s)
        old_t, t = (t, old_t - quotient * t)

    return old_s, old_t, old_r # a, b, g
```

Hàm `xgcd(x, y)` thực hiện thuật toán Euclid mở rộng (Extended Euclidean Algorithm) để tìm bộ ba số (a, b, g) thỏa mãn:

$ax + by = g$

Trong đó:
- x và y là hai số nguyên đầu vào
- $g = gcd(x, y)$ là ước chung lớn nhất của x và y
- a và b là các hệ số Bézout

Công thức toán học:

1) Đầu vào: Hai số nguyên x và y

2) Đầu ra: Bộ ba số (a, b, g) thỏa mãn phương trình Bézout:
   $ax + by = g$
   
   Trong đó $g = gcd(x, y)$

3) Tính chất:
   - Nếu $x = y = 0$, kết quả là $(0, 0, 0)$
   - Nếu $x = 0$ và $y \neq 0$, kết quả là $(0, 1, |y|)$
   - Nếu $x \neq 0$ và $y = 0$, kết quả là $(1, 0, |x|)$
   - Trong trường hợp tổng quát, $|a| \leq |y|/(2g)$ và $|b| \leq |x|/(2g)$

4) Ý nghĩa:
   - Hàm này không chỉ tìm ước chung lớn nhất g của x và y
   - Mà còn tìm được các hệ số Bézout a và b
   - Các hệ số này có thể được sử dụng trong nhiều ứng dụng số học, bao gồm tính nghịch đảo modulo và giải các phương trình Diophantine tuyến tính

Hàm trả về bộ ba (old_s, old_t, old_r) tương ứng với (a, b, g) trong công thức trên.

## Trường hữu hạn

```python
# algebra.py
class FieldElement:
    def __init__( self, value, field ):
        self.value = value
        self.field = field

    def __add__( self, right ):
        return self.field.add(self, right)

    def __mul__( self, right ):
        return self.field.multiply(self, right)

    def __sub__( self, right ):
        return self.field.subtract(self, right)

    def __truediv__( self, right ):
        return self.field.divide(self, right)

    def __neg__( self ):
        return self.field.negate(self)

    def inverse( self ):
        return self.field.inverse(self)

    # modular exponentiation -- be sure to encapsulate in parentheses!
    def __xor__( self, exponent ):
        acc = FieldElement(1, self.field)
        val = FieldElement(self.value, self.field)
        for i in reversed(range(len(bin(exponent)[2:]))):
            acc = acc * acc
            if (1 << i) & exponent != 0:
                acc = acc * val
        return acc

    def __eq__( self, other ):
        return self.value == other.value

    def __neq__( self, other ):
        return self.value != other.value

    def __str__( self ):
        return str(self.value)

    def __bytes__( self ):
        return bytes(str(self).encode())

    def is_zero( self ):
        if self.value == 0:
            return True
        else:
            return False

class Field:
    def __init__( self, p ):
        self.p = p

    def zero( self ):
        return FieldElement(0, self)

    def one( self ):
        return FieldElement(1, self)

    def multiply( self, left, right ):
        return FieldElement((left.value * right.value) % self.p, self)

    def add( self, left, right ):
        return FieldElement((left.value + right.value) % self.p, self)

    def subtract( self, left, right ):
        return FieldElement((self.p + left.value - right.value) % self.p, self)

    def negate( self, operand ):
        return FieldElement((self.p - operand.value) % self.p, self)

    def inverse( self, operand ):
        a, b, g = xgcd(operand.value, self.p)
        return FieldElement(((a % self.p) + self.p) % self.p, self)

    def divide( self, left, right ):
        assert(not right.is_zero()), "divide by zero"
        a, b, g = xgcd(right.value, self.p)
        return FieldElement(left.value * a % self.p, self)

    def main():
        p = 1 + 407 * ( 1 << 119 ) # 1 + 11 * 37 * 2^119
        return Field(p)

    def generator( self ):
        assert(self.p == 1 + 407 * ( 1 << 119 )), "Do not know generator for other fields beyond 1+407*2^119"
        return FieldElement(85408008396924667383611388730472331217, self)

    def primitive_nth_root( self, n ):
        if self.p == 1 + 407 * ( 1 << 119 ):
            assert(n <= 1 << 119 and (n & (n-1)) == 0), "Field does not have nth root of unity where n > 2^119 or not power of two."
            root = FieldElement(85408008396924667383611388730472331217, self)
            order = 1 << 119
            while order != n:
                root = root^2
                order = order/2
            return root
        else:
            assert(False), "Unknown field, can't return root of unity."
            
    def sample( self, byte_array ):
        acc = 0
        for b in byte_array:
            acc = (acc << 8) ^ int(b)
        return FieldElement(acc % self.p, self)
```
1. Lớp FieldElement:

Đây là một lớp đại diện cho một phần tử trong trường hữu hạn (finite field). Mỗi phần tử có một giá trị và thuộc về một trường cụ thể.

- Các phép toán cơ bản (+, -, *, /) được định nghĩa thông qua các phương thức magic (__add__, __sub__, __mul__, __truediv__). Chúng đều gọi đến các phương thức tương ứng của lớp Field.

- Phép lũy thừa modulo (__xor__) được thực hiện bằng thuật toán bình phương và nhân (square-and-multiply):

  $a^n \bmod p = \begin{cases}
  ((a^2)^{n/2} \bmod p) & \text{nếu n chẵn} \\
  (a \cdot (a^2)^{(n-1)/2} \bmod p) & \text{nếu n lẻ}
  \end{cases}$

2. Lớp Field:

Đại diện cho một trường hữu hạn với modulo p.

- Phép cộng: $(a + b) \bmod p$
- Phép trừ: $(p + a - b) \bmod p$ (để tránh kết quả âm)
- Phép nhân: $(a \cdot b) \bmod p$
- Phép chia: $(a \cdot b^{-1}) \bmod p$, trong đó $b^{-1}$ là nghịch đảo modular của b

- Phương thức inverse sử dụng thuật toán Euclid mở rộng (xgcd) để tìm nghịch đảo modular:
  $ax + py = 1$ (phương trình Bézout)
  Khi đó x là nghịch đảo modular của a modulo p

- primitive_nth_root tìm căn nguyên thủy bậc n của 1:
  $\omega^n \equiv 1 \pmod{p}$
  $\omega^k \not\equiv 1 \pmod{p}$ với mọi $k < n$

- sample chuyển đổi một mảng byte thành một phần tử trường:
  Sử dụng phép dịch bit và XOR để tạo một số nguyên lớn, sau đó lấy modulo p.

Mã này cung cấp một cài đặt đầy đủ cho các phép toán trên trường hữu hạn, rất hữu ích trong mật mã học và lý thuyết số.
