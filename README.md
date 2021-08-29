# AproximadorDeZerosDeFuncoes
Alguns CLI para efetuar a aproximação numérica de funções, com base nas aulas de Cálculo Numérico. Por hora, encontran-se os algorítimos:   
- Bissecção
- Newton Raphson

## Como Utilizar

### Bissecção

O executavel espera quatro argumentos:

**Limite Inferior:** Indica o valor inferior da aproximação inicial. Deve ser um número.   
**Limite Superior:** Indica o valor superior da aproximação inicial. Deve ser um número.   
**Equação:** Qual a equação que será aproximada. Essa pode ser qualquer equação válida dentro dos limites do pacote meval.   
**Limite Iterações:** Indica o limite da aproximação do método.   

Caso exista algum erro em algum dos argumentos, o CLI vai mandar de volta um erro.

Exemplos de uso:

./AproximadorDeZerosDeFuncoes.exe 2 3 "x^2-5" 30

Irá tentar aproximar o zero da função "x^2-5" entre os limites 2(inferior) e 3(superior). É equivalente a tentar achar o resultado de sqrt(5). O programa não faz garantia de zeros entre os limites, nem a unicidade, por isso, é preciso mandar valores que garantam a existência e unicidade, caso contrário o algorítimo não irá dar resultados.

### Newton Raphson

O executavel espera quatro argumentos:

**Valor Inicial:** Indica o valor inicial da iteração. Quanto mais próximo do zero real, mais rápido é a convergência.   
**Limite Iterações:** Indica o limite da aproximação do método.   
**Equação:** Qual a equação que será aproximada. Essa pode ser qualquer equação válida dentro dos limites do pacote meval.   
**Derivada:** Qual a derivada equação que será aproximada. Essa pode ser qualquer equação válida dentro dos limites do pacote meval.   


Caso exista algum erro em algum dos argumentos, o CLI vai mandar de volta um erro.


