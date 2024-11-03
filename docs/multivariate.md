# Đa thức đa biến

```python
# multivariate.py
from univariate import *

class MPolynomial:
    def __init__( self, dictionary ):
        # Multivariate polynomials are represented as dictionaries with exponent vectors
        # as keys and coefficients as values. E.g.:
        # f(x,y,z) = 17 + 2xy + 42z - 19x^6*y^3*z^12 is represented as:
        # {
        #     (0,0,0) => 17,
        #     (1,1,0) => 2,
        #     (0,0,1) => 42,
        #     (6,3,12) => -19,
        # }
        self.dictionary = dictionary

    # Các phương thức khác...

```

Lớp `MPolynomial` đại diện cho một đa thức đa biến với các phương thức chính sau:

1. `__init__(self, dictionary)`: Khởi tạo đa thức đa biến với một từ điển, trong đó khóa là vector mũ và giá trị là hệ số.

2. `zero()`: Trả về đa thức không (đa thức bằng 0).

3. `__add__(self, other)`: Cộng hai đa thức đa biến, $P(x_1,...,x_n) + Q(x_1,...,x_n)$.

4. `__mul__(self, other)`: Nhân hai đa thức đa biến, $P(x_1,...,x_n) \cdot Q(x_1,...,x_n)$.

5. `__sub__(self, other)`: Trừ hai đa thức đa biến, $P(x_1,...,x_n) - Q(x_1,...,x_n)$.

6. `__neg__(self)`: Trả về đa thức đối, $-P(x_1,...,x_n)$.

7. `__xor__(self, exponent)`: Tính lũy thừa của đa thức đa biến, $P(x_1,...,x_n)^n$.

8. `constant(element)`: Tạo đa thức hằng số.

9. `is_zero(self)`: Kiểm tra xem đa thức có bằng 0 không.

10. `variables(num_variables, field)`: Trả về danh sách các đa thức đa biến đại diện cho từng biến.

11. `evaluate(self, point)`: Tính giá trị của đa thức tại một điểm.

12. `evaluate_symbolic(self, point)`: Tính giá trị của đa thức tại một điểm, trả về một đa thức một biến.

13. `lift(polynomial, variable_index)`: Chuyển đổi một đa thức một biến thành đa thức đa biến.

Các phép toán trên đa thức đa biến được thực hiện bằng cách kết hợp các phép toán trên các hệ số và vector mũ tương ứng. Ví dụ, khi cộng hai đa thức, ta cộng các hệ số của các số hạng có cùng vector mũ.

Lớp này cho phép thực hiện các phép toán đại số trên đa thức đa biến một cách hiệu quả, hỗ trợ các phép tính tượng trưng và số học trên đa thức đa biến.

