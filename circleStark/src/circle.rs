use lambdaworks_math::elliptic_curve::edwards::curves::bandersnatch::field;
use lambdaworks_math::field::element::FieldElement;
use lambdaworks_math::field::fields::mersenne31::field::Mersenne31Field as M31;
struct CirclePoint {
    x: FieldElement<M31>,
    y: FieldElement<M31>,
}
trait CircleImpl {
    fn double(&self) -> CirclePoint;
}

impl CircleImpl for CirclePoint {
    // (x,y) ->  (2x^2-1 , 2*x*y)
    fn double(&self) -> CirclePoint {
        return CirclePoint {
            x: self.x.square().double() - FieldElement::one(),
            y: (self.y * self.x).double(),
        };
    }
}

pub fn G() -> CirclePoint {
    CirclePoint {
        x: 1268011823.into(),
        y: 2.into(),
    }
}

pub fn Z() -> CirclePoint {
    CirclePoint {
        x: 1.into(),
        y: 0.into(),
    }
}
