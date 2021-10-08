library STD;
use STANDARD.ALL;


entity questao1 is
port(
    a, b : IN BIT; -- variaveis de entrada
    x : OUT BIT; -- variaveis de saida
     );
end questao1;

architecture q1 of questao1 is
Begin
 --   x <= NOT(NOT(a));
    x <= NOT(NOT(b));
 --   x <= NOT(NOT(a AND b));
 --   x <= NOT(NOT(a AND b));
 --   x <= NOT(a AND b);
 --  x <= NOT(a OR b);
end q1;