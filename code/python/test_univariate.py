from univariate import *
import os
import time

def test_distributivity():
    field = Field.main()
    zero = field.zero()
    one = field.one()
    two = FieldElement(2, field)
    five = FieldElement(5, field)

    a = Polynomial([one, zero, five, two])
    b = Polynomial([two, two, one])
    c = Polynomial([zero, five, two, five, five, one])

    lhs = a * (b + c)
    rhs = a * b + a * c
    assert(lhs == rhs), "distributivity fails for polynomials: {} =/= {}".format(lhs.__str__(), rhs.__str__())

    print("univariate polynomial distributivity success \\o/")

def test_division():
    field = Field.main()
    zero = field.zero()
    one = field.one()
    two = FieldElement(2, field)
    five = FieldElement(5, field)

    a = Polynomial([one, zero, five, two])
    b = Polynomial([two, two, one])
    c = Polynomial([zero, five, two, five, five, one])

    # a should divide a*b, quotient should be b
    quo, rem = Polynomial.divide(a*b, a)
    assert(rem.is_zero()), "fail division test 1"
    assert(quo == b), "fail division test 2"

    # b should divide a*b, quotient should be a
    quo, rem = Polynomial.divide(a*b, b)
    assert(rem.is_zero()), "fail division test 3"
    assert(quo == a), "fail division test 4"

    # c should not divide a*b
    quo, rem = Polynomial.divide(a*b, c)
    assert(not rem.is_zero()), "fail division test 5"

    # ... but quo * c + rem == a*b
    assert(quo * c + rem == a*b), "fail division test 6"

    print("univariate polynomial division success \\o/")

def test_interpolate():
    field = Field.main()
    zero = field.zero()
    one = field.one()
    two = FieldElement(2, field)
    five = FieldElement(5, field)
    
    values = [five, two, two, one, five]
    domain = [FieldElement(i, field) for i in range(1, 6)]

    poly = Polynomial.interpolate_domain(domain, values)

    for i in range(len(domain)):
        assert(poly.evaluate(domain[i]) == values[i]), "fail interpolate test 1"

    # evaluation in random point is nonzero with high probability
    assert(poly.evaluate(FieldElement(363, field)) != zero), "fail interpolate test 2"

    assert(poly.degree() == len(domain)-1), "fail interpolate test 3"

    print("univariate polynomial interpolate success \\o/")

def test_zerofier():
    field = Field.main()

    for trial in range(0, 100):
        degree = max(1, int(os.urandom(1)[0]) % 10)  # Đảm bảo degree ít nhất là 1
        domain = []
        while len(domain) != degree:
            new = field.sample(os.urandom(17))
            if new not in domain:
                domain.append(new)

        print(f"Trial {trial + 1}: degree = {degree}, domain size = {len(domain)}")

        zerofier = Polynomial.zerofier_domain(domain)

        assert zerofier.degree() == degree, f"zerofier has degree {zerofier.degree()}, expected {degree}"

        for d in domain:
            assert zerofier.evaluate(d) == field.zero(), f"zerofier does not evaluate to zero at {d}"

        random = field.sample(os.urandom(17))
        while random in domain:
            random = field.sample(os.urandom(17))

        assert zerofier.evaluate(random) != field.zero(), f"zerofier evaluates to zero at random point {random}"

    print("univariate zerofier test success \\o/")


start_time = time.time()

test_distributivity()
test_division()
test_interpolate()
test_zerofier()

end_time = time.time()
execution_time = end_time - start_time

print(f"Thời gian chạy: {execution_time:.2f} giây")