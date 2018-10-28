# BIRLSCRIPT/MBR

## Vá com cuidado, amigo.
Esse fork quebra compatibilidade com a API do interpretador original,
por fazer parte do esforço de expôr uma interface em núvem.
Vá ao [Repositório Oficial](https://github.com/birlscript/birlscript)
do BirlScript se quiser a real, o oficial, o inigualável. Se quiser
ficar, é melhor que tenha em mente que esse aqui vai estar em descompasso
com a língua por enquanto ou pode ser que seja reincorporado ao projeto
original.

## Agora pro que importa.

É a linguagem de scripting dos programadores codebuilders! Dos que não saem de casa
sem codar pra caralho! Se você ta achando que é moleza, ta enganado, aqui não é
café com músculo, aqui se constrói código, porra!

Se você é um programador mutante e se sente apto pro desafio, vá em frente!
Ajude no desenvolvimento do projeto com ideias, código e muita fibra!

Use o interpretador com a flag *-a* ou *--ajuda-o-maluco-ta-doente* para ver uma lista
de opções que podem ser passadas.

[Editor Online](https://birlscript.github.io/), pra rodar código na web

## Versão 2.0.0 alpha

*© 2016-2018 Rafael Rodrigues Nakano. Contato: lazpeng@gmail.com*

***Removido brief sobre a linguagem por conta do novo backend (que, diferente do atual, não é uma bagunça mal-feita) que deve
mudar bastante coisa na própria língua. Exemplos podem sofrer mudanças também***

# Exemplos

## Fatorial
```python
JAULA FATORIAL (NUMERO: BATATA DOCE, ATUAL : BATATA DOCE)
    E ELE QUE A GENTE QUER: NUMERO, 1
    MENOR OU E MEMO:
        BIRL: ATUAL
    FIM
    BORA: NUMERO, NUMERO - 1
    BORA: ATUAL, ATUAL * NUMERO
    E HORA DO: FATORIAL, NUMERO, ATUAL
    BIRL: TREZE
SAINDO DA JAULA

JAULA SHOW
    VEM: NUMERO, 4
    CE QUER VER: "FATORIAL DE ", NUMERO, " É: "
    E HORA DO: FATORIAL, NUMERO, NUMERO
    CE QUER VER ISSO: TREZE
SAINDO DA JAULA
```

## Sequência fibonacci
```python
JAULA FIBONACCI(NUMERO: BATATA DOCE)
    E ELE QUE A GENTE QUER: NUMERO, 1
    MENOR OU E MEMO:
        BIRL: NUMERO
    FIM
    VEM: RESULTADO, 0
    E HORA DO: FIBONACCI, NUMERO - 1
    BORA: RESULTADO, TREZE
    E HORA DO: FIBONACCI, NUMERO - 2
    BIRL: RESULTADO + TREZE
SAINDO DA JAULA

JAULA PRINTA_FIBONACCI(TOTAL: BATATA DOCE, VEZES: BATATA DOCE)
    E ELE QUE A GENTE QUER: TOTAL, VEZES
    E ELE MEMO:
        BIRL
    FIM
    E HORA DO: FIBONACCI, TOTAL
    CE QUER VER ISSO: TREZE
    E HORA DO: PRINTA_FIBONACCI, TOTAL + 1, VEZES
SAINDO DA JAULA

JAULA SHOW
    VEM: VEZES, 13
    E HORA DO: PRINTA_FIBONACCI, 0, VEZES
SAINDO DA JAULA

```

## Hello world, cumpade!
```python
# A JAULA SHOW é opcional, codigos podem ser executados fora de uma JAULA
# porem uma jaula (no caso, a show) permite que se faça uso de recursão, o que não é disponivel em comandos globais
CE QUER VER ISSO: "BORA, " + CUMPADE + "!" # O operador + em strings só pode ser usado com outra string
```

## Funções e condicionais
```python
JAULA OUTRO # Declaração da JAULA outro
    CE QUER VER ISSO: "estou em outra"
SAINDO DA JAULA # Fim da declaração de OUTRO

JAULA DIFERENTE() # No caso de nao possuir parametros, o uso de parenteses é opcional
    CE QUER VER ISSO: "deu diferente"
SAINDO DA JAULA

JAULA SHOW
    E HORA DO: OUTRO # Passa a execução pra OUTRO
    VEM: MUTANTE, "FIBRA"
    E ELE QUE A GENTE QUER: MUTANTE, "AGUA COM MUSCULO" # Compara MUTANTE com "AGUA COM MUSCULO"
    NUM E ELE:
        É HORA DO: DIFERENTE # Caso seja diferente, execute DIFERENTE
    FIM
SAINDO DA JAULA
```

##Documentação
***Em breve***
