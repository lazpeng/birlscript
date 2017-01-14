# BIRLSCRIPT

É a linguagem de scripting dos programadores codebuilders! Dos que não saem de casa
sem codar pra caralho! Se você ta achando que é moleza, ta enganado, aqui não é
café com músculo, aqui se constrói código, porra!

Se você é um programador mutante e se sente apto pro desafio, vá em frente!
Ajude no desenvolvimento do projeto com ideias, código e muita fibra!

Use o interpretador com a flag *-a* ou *--ajuda-o-maluco-ta-doente* para ver uma lista
de opções que podem ser passadas. Use *-t* ou *--tudo-cumpade* para uma lista de comandos.

[Editor Online](https://birlscript.github.io/), teste agora mesmo!

## Versão 1.2.0

*Copyright© 2016, 2017 Rafael Rodrigues Nakano. Contato: lazpeng@gmail.com*

Você tem acesso a globais (variáveis constantes diponíveis pra todo o programa), seções
(que são como funções, porém muito mais primitivas e limitadas) e os comandos, que funcionam
de forma similar que em BASIC, só que com frases e dizeres do mestre bodybuilder (alguns sim,
outros não. Optei por deixar o que fizesse ao menos o mínimo de sentido).

Algumas variáveis padrão são definidas 1 - no começo do programa ou 2 - no começo de cada seção.
As definidas no começo do programa são "CUMPADE" que contém seu nome de usuário (ou CUMPADE se não encontrado), "UM" que contém
o valor 1, "BODYBUILDER" que contem a string "BAMBAM". No momento, apenas uma é definida no inicio de cada seção,
que é a "JAULA" que contem o nome da seção. Todas essas variáveis são constantes.

Comandos como o BORA (que modifica um valor) e BORA, CUMPADE (e BORA, CUMPADE!!!) (que pedem entrada do usuário)
esperam que a variável passada já exista.
Quando uma variável é criada sem valor (VEM), seu valor é Nulo (antigamente era 0). Então qualquer
expressão envolvendo essas variáveis pode resultar em erro.

O Comando BIRL (de retorno) tem duas peculiaridades: 1º - Se chamado fora de alguma função (seção global)
o programa termina sua execução. Se o valor passado pra ele nessa situação for um número, ele retorna pro SO
com esse código. 2º - Você não precisa retornar um valor. BIRL pode ser chamado sem sequer os dois pontos,
entretanto, talvez um outro código espere um valor na variavel TREZE (que é onde o retorno de uma seção fica
guardado), que inicia a seção com valor Nulo e pode causar problemas.

Uma variável que é definida a cada seção é TREZE, mas não é listada junto com as outras por não ser constante.
Essa variável muda (ou não) a cada chamada de função (É HORA DO) (caso a última função não retorne nenhum valor,
o valor da variável TREZE não é alterado).

# Exemplos

***Mais exemplos em breve***

## Variáveis
```python
JAULA SHOW
  VEM: MONSTRO # Declara variavel com o valor padrão, 0
  VEM, CUMPADE: IBIRAPUERA, "BIRL" # Declara IBIRAPUERA com valor "BIRL"
  BORA: MONSTRO, 2 # Da o valor 2 para MONSTRO
  BORA: MONSTRO, MONSTRO * 2 # Multiplica o valor de MONSTRO por 2
  CE QUER VER ISSO: "MONSTRO: ", MONSTRO, " IBIRAPUERA: " + IBIRAPUERA
SAINDO DA JAULA
```

## Variáveis padrão
```python
# Existem varias variaveis que são inicializadas por padrão, que são, por sua vez, constantes.
# Elas são:
CE QUER VER ISSO: CUMPADE # Contem seu nome de usuario
CE QUER VER ISSO: UM # Teste, contem o valor 1
CE QUER VER ISSO: BODYBUILDER # Outra constante, contem o valor BAMBAM

JAULA SHOW() # Parenteses opcionais se nao houver parametros, tanto na declaração quanto na chamada
    CE QUER VER ISSO: JAULA # Contem o nome da JAULA atual, no caso, printa SHOW
SAINDO DA JAULA
```

## Hello world, cumpade!
```python
# A JAULA SHOW é opcional, codigos podem ser executados fora de uma JAULA
# porem uma jaula (no caso, a show) permite que se faça uso de recursão,
# o que não é disponivel em comandos globais
CE QUER VER ISSO: "BORA, " + CUMPADE + "!"

```

## Seções e condicionais
```python
JAULA OUTRO # Declaração da JAULA outro
  CE QUER VER ISSO: "estou em outra"
SAINDO DA JAULA # Fim da declaração de OUTRO

JAULA DIFERENTE() # No caso de nao possuir parametros, o uso de parenteses é opcional,
# tanto na declaração quanto de chamada
  CE QUER VER ISSO: "deu diferente"
SAINDO DA JAULA

JAULA SHOW
  E HORA DO: OUTRO() # Passa a execução pra OUTRO
  VEM, CUMPADE: MUTANTE, "FIBRA"
  E ELE QUE A GENTE QUER: MUTANTE, "AGUA COM MUSCULO" # Compara MUTANTE com "AGUA COM MUSCULO"
  NUM E ELE: É HORA DO: DIFERENTE # Caso seja diferente, execute DIFERENTE
SAINDO DA JAULA
```

## Globais
```python
IBIRAPUERA GLOBAL_VARIAVEL "PRIMEIRO VALOR" # Globais com IBIRAPUERA podem ser alterados
SAI DE CASA GLOBAL_CONSTANTE "UNICO VALOR" # Globais com SAI DE CASA não podem ter ser alterados

CE QUER VER ISSO: GLOBAL_VARIAVEL # Printa PRIMEIRO VALOR
BORA: GLOBAL_VARIAVEL, "SEGUNDO VALOR" # Altera o valor da global
CE QUER VER ISSO: GLOBAL_VARIAVEL # Printa SEGUNDO VALOR
CE QUER VER ISSO: GLOBAL_CONSTANTE # Printa UNICO VALOR
#BORA: GLOBAL_CONSTANTE, "ERRO" # Descomente essa linha e verá o erro na execução
```

## Fatorial
```python
# TRAPEZIO DESCENDENTE: tipo para NUMERO
# FIBRA: tipo para texto/string

JAULA FATORIAL (NUMERO: TRAPEZIO DESCENDENTE, ATUAL : TRAPEZIO DESCENDENTE)
    E ELE QUE A GENTE QUER: NUMERO, 1
    MENOR OU E MEMO: BIRL: ATUAL
    BORA: NUMERO, NUMERO - 1
    BORA: ATUAL, ATUAL * NUMERO
    E HORA DO: FATORIAL(NUMERO, ATUAL)
    BIRL: TREZE
SAINDO DA JAULA

JAULA SHOW
    VEM, CUMPADE: NUMERO, 4
    CE QUER VER: "FATORIAL DE ", NUMERO, " É: "
    E HORA DO: FATORIAL(NUMERO, NUMERO)
    CE QUER VER ISSO: TREZE
SAINDO DA JAULA
```

## Não quero falar com bandeirantes!
```python
JAULA SHOW
    VEM: EMISSORA # Cria variavel EMISSORA
    BORA, CUMPADE!!!: EMISSORA # Guarda valor da entrada em EMISSORA com letras maiusculas
    E ELE QUE A GENTE QUER: EMISSORA, "BANDEIRANTES" # Compara com bandeirantes
    E ELE MEMO: CE QUER VER ISSO: "NÃO QUERO FALAR COM BANDEIRANTES" # Caso seja igual, execute bandeirantes
    NUM E ELE: CE QUER VER ISSO: "COM " + EMISSORA + " EU FALO" # Diferente, execute outro
SAINDO DA JAULA
```

## Sequencia fibonacci
```python
JAULA FIBONACCI(NUMERO: TRAPEZIO DESCENDENTE)
E ELE QUE A GENTE QUER: NUMERO, 1
MENOR OU E MEMO: BIRL: NUMERO
VEM: RESULTADO
E HORA DO: FIBONACCI(NUMERO - 1)
BORA: RESULTADO, TREZE
E HORA DO: FIBONACCI(NUMERO - 2)
BIRL: RESULTADO + TREZE
SAINDO DA JAULA

JAULA PRINTA_FIBONACCI(TOTAL: TRAPEZIO DESCENDENTE, VEZES: TRAPEZIO DESCENDENTE)
E ELE QUE A GENTE QUER: TOTAL, VEZES
E ELE MEMO: BIRL
E HORA DO: FIBONACCI(TOTAL)
CE QUER VER ISSO: TREZE
E HORA DO: PRINTA_FIBONACCI(TOTAL + 1, VEZES)
SAINDO DA JAULA

JAULA SHOW
VEM, CUMPADE: VEZES, 13
E HORA DO: PRINTA_FIBONACCI(0, VEZES)
SAINDO DA JAULA
```