# BIRLSCRIPT

[![Status do projeto](https://www.openhub.net/p/birlscript/widgets/project_thin_badge?format=gif&ref=Thin+badge)](https://www.openhub.net/p/birlscript)

É a linguagem de scripting dos programadores codebuilders! Dos que não saem de casa
sem codar pra caralho! Se você ta achando que é moleza, ta enganado, aqui não é
café com músculo, aqui se constrói código, porra!

Se você é um programador mutante e se sente apto pro desafio, vá em frente!
Ajude no desenvolvimento do projeto com ideias, código e muita fibra!

Use o interpretador com a flag *-a* ou *--ajuda-o-maluco-ta-doente* para ver uma lista
de opções que podem ser passadas. Use *-e* ou *--ele-que-a-gente-quer* junto com o nome
de um comando para ver mais sobre ele, ou *-t* ou *--tudo-cumpade* para uma lista de comandos.

## Versão 1.1.5

## Builds:
* Windows (64-bits): Disponível
* Windows (32-bits): Disponível
* Linux   (64-bits): Disponível
* Linux   (32-bits): Em breve
* OS X    (64-bits): Não disponível[²]
* OS X    (32-bits): Não disponível[²]

[²] - Não tenho um Mac pra compilar o programa pra tal, então não tenho como fornecer os binários. Se alguém tiver e quiser compilar eu posso fornecer instruções.

*Copyleft(ɔ) 2016 Rafael R Nakano. Nenhum direito reservado.*
*Contato: lazpeng@gmail.com*

## Sobre
BIRLSCRIPT é uma espécie de dialeto [BASIC](https://pt.wikipedia.org/wiki/BASIC) com algumas pequenas (ou grandes)
modificações pra fazer a vida de quem tá programando ou de quem tá escrevendo o parser
(eu mereço) mais fácil. Há algumas limitações claras e 95% delas serão sanadas no futuro,
com exceção de algumas que *não fazem sentido*, *dariam muito trabalho pra implementar* ou
*tem outras formas de se chegar no mesmo resultado*.

Você tem acesso a globais (variáveis constantes diponíveis pra todo o programa), seções
(que são como funções, porém muito mais primitivas e limitadas) e os comandos, que funcionam
de forma similar que em BASIC, só que com frases e dizeres do mestre bodybuilder (alguns sim,
outros não. Optei por deixar o que fizesse ao menos o mínimo de sentido).

# Exemplos

## Variáveis
```python
JAULA SHOW
  VEM: MONSTRO # Declara variavel com o valor padrão, 0
  VEM, CUMPADE: IBIRAPUERA, "BIRL" # Declara IBIRAPUERA com valor "BIRL"
  BORA: MONSTRO, 2 # Da o valor 2 para MONSTRO
  BORA: MONSTRO, MONSTRO * 2 # Multiplica o valor de MONSTRO por 2
  CE QUER VER ISSO: "MONSTRO: ", MONSTRO, "IBIRAPUERA: " + IBIRAPUERA
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
# porem uma jaula (no caso, a show) permite que se faça uso de recursão, o que não é disponivel em comandos globais
CE QUER VER ISSO: "BORA, " + CUMPADE + " !" # O operador + em strings só pode ser usado com outra string
```

## Seções e condicionais
```python
JAULA OUTRO # Declaração da JAULA outro
  CE QUER VER ISSO: "estou em outra"
SAINDO DA JAULA # Fim da declaração de OUTRO

JAULA DIFERENTE() # No caso de nao possuir parametros, o uso de parenteses é opcional, tanto na declaração quanto de chamada
  CE QUER VER ISSO: "deu diferente"
SAINDO DA JAULA

JAULA SHOW
  E HORA DO: OUTRO() # Passa a execução pra OUTRO
  VEM, CUMPADE: MUTANTE, "FIBRA"
  E ELE QUE A GENTE QUER: MUTANTE, "AGUA COM MUSCULO" # Compara MUTANTE com "AGUA COM MUSCULO"
  NUM E ELE: DIFERENTE # Caso seja diferente, execute DIFERENTE
SAINDO DA JAULA
```

## Globais
```python
IBIRAPUERA: GLOBAL_VARIAVEL: "PRIMEIRO VALOR" # Globais declarados com IBIRAPUERA podem ter seu valor alterado
SAI DE CASA: GLOBAL_CONSTANTE: "UNICO VALOR" # Globais declarados com SAI DE CASA não podem ter seu valor alterado

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

JAULINHA PRINTA
    BIRL: ATUAL # Retorna o valor pro main
SAINDO DA JAULA

JAULA FATORIAL (NUMERO: TRAPEZIO DESCENDENTE, ATUAL : TRAPEZIO DESCENDENTE)
    E ELE QUE A GENTE QUER: NUMERO, 1
    MENOR OU E MEMO: PRINTA
    BORA: NUMERO, NUMERO - 1
    BORA: ATUAL, ATUAL * NUMERO
    E HORA DO: FATORIAL(NUMERO, ATUAL)
SAINDO DA JAULA

JAULA SHOW
    VEM, CUMPADE: NUMERO, 4
    CE QUER VER: "FATORIAL DE ", NUMERO, " É: "
    E HORA DO: FATORIAL(NUMERO, NUMERO)
    CE QUER VER ISSO: TREZE # Nome do valor de retorno padrão
SAINDO DA JAULA
```

## Não quero falar com bandeirantes!
```python
JAULA BANDEIRANTES
    CE QUER VER ISSO: "NÃO QUERO FALAR COM BANDEIRANTES"
SAINDO DA JAULA

JAULA OUTRA (EMISSORA : FIBRA)
    CE QUER VER ISSO: "COM " + EMISSORA + " EU FALO"
SAINDO DA JAULA

JAULA SHOW
    VEM: EMISSORA # Cria variavel EMISSORA
    BORA, CUMPADE!!!: EMISSORA # Guarda valor da entrada em EMISSORA com letras maiusculas
    E ELE QUE A GENTE QUER: EMISSORA, "BANDEIRANTES" # Compara com bandeirantes
    E ELE MEMO: BANDEIRANTES() # Caso seja igual, execute bandeirantes
    NUM E ELE: OUTRA(EMISSORA) # Diferente, execute outro
SAINDO DA JAULA
```
