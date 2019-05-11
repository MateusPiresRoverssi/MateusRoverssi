use std::f64;

trait Verificar {
    fn verificar(&self);
}

struct Aluno {
    ra: u32,
    p1: f64,
    ma1: f64,
    mb1: f64,
    p2: f64,
    ma2: f64,
    mb2: f64,
    qtdAulas: f64,
    qtdFaltas: f64
}

impl Verificar for Aluno {
    fn verificar(&self)  {
        let a1: f64 = self.p1 * 0.7 + self.ma1 * 0.2 + self.mb1 * 0.1;
        let a2: f64 = self.p2 * 0.7 + self.ma2 * 0.2 + self.mb2 * 0.1;
        let mediaFinal: f64 = (a1 + a2 * 2.0) / 3.0;
        let presenca: f64 =  self.qtdAulas / self.qtdFaltas;
        if mediaFinal >= 5.0 && presenca >= 0.75 {
            println!("Aluno {} aprovado!", self.ra);
        } else if mediaFinal < 5.0 && mediaFinal >= 3.0 && presenca >= 0.75 {
            println!("Aluno {} reprovado por nota!", self.ra);
        } else {
            println!("Aluno {} reprovado por falta!", self.ra);
        }
    }
}

fn main() {
    println!("Hello, world!");
    let aluno1 = Aluno{
        ra: 123,
        p1: 6.0,
        ma1: 4.0,
        mb1: 5.0,
        p2: 1.0,
        ma2: 3.0,
        mb2: 8.0,
        qtdAulas: 40.0,
        qtdFaltas: 2.0
    };

   let aluno2 = Aluno{
        ra: 65,
        p1: 1.0,
        ma1: 4.0,
        mb1: 5.0,
        p2: 1.0,
        ma2: 3.0,
        mb2: 8.0,
        qtdAulas: 40.0,
        qtdFaltas: 38.0
    };
    
    let aluno3 = Aluno{
        ra: 456,
        p1: 6.0,
        ma1: 4.0,
        mb1: 5.0,
        p2: 10.0,
        ma2: 3.0,
        mb2: 8.0,
        qtdAulas: 40.0,
        qtdFaltas: 2.0
    };
   
    let alunos: Vec<&Verificar> = vec![&aluno1,&aluno2,&aluno3];
    for a in &alunos {
        a.verificar();
    }

}
