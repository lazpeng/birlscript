# BIRLSCRIPT
É a linguagem de scripting dos programadores codebuilders! Dos que não saem de casa
sem codar pra caralho! Se você ta achando que é moleza, ta enganado, aqui não é
café com músculo, aqui se constrói código, porra!

Se você é um programador mutante e se sente apto pro desafio, vá em frente!
Ajude no desenvolvimento do projeto com ideias, código e muita fibra!

Use o interpretador com a flag *-a* ou *--ajuda-o-maluco-ta-doente* para ver uma lista
de opções que podem ser passadas. Use *-e* ou *--ele-que-a-gente-quer* junto com o nome
de um comando para ver mais sobre ele, ou *-t* ou *--tudo-cumpade* para uma lista de comandos.

Builds estarão disponiveis na pasta *bin* a partir da versão 1.x BETA, que estará funcional.

## Versão 1.0.0 BETA

## Builds:
*Windows (64-bits): Disponível
*Windows (32-bits): Em breve
*Linux   (64-bits): Em breve
*Linux   (32-bits): Em breve
*OS X    (64-bits): Não disponível
*OS X    (32-bits): Não disponível

*Copyleft(ɔ) 2016 Rafael R Nakano. Nenhum direito reservado.*
*Contato: lazpeng@gmail.com*

## Sobre
BIRLSCRIPT (ou BIRLASM) é uma espécie de dialeto assembly[¹] com algumas pequenas (ou grandes)
modificações pra fazer a vida de quem tá programando ou de quem tá escrevendo o parser
(eu mereço) mais fácil. Há algumas limitações claras e 95% delas serão sanadas no futuro,
com exceção de algumas que *não fazem sentido*, *dariam muito trabalho pra implementar* ou
*tem outras formas de se chegar no mesmo resultado*.

Você tem acesso a globais (variáveis constantes diponíveis pra todo o programa), seções
(que são como funções, porém muito mais primitivas e limitadas) e os comandos, que funcionam
de forma similar que em assembly[¹], só que com frases e dizeres do mestre bodybuilder (alguns sim,
outros não. Optei por deixar o que fizesse ao menos o mínimo de sentido).

[¹] - *Assembly aqui se diz respeito à linguagem assembly para a arquitetura x86, dialeto NASM*

# Exemplos

## Variáveis
```rust
JAULA SHOW
  VEM: MONSTRO ; Declara variavel com o valor padrão, 0
  VEM, PORRA: IBIRAPUERA, "BIRL" ; Declara IBIRAPUERA com valor "BIRL"
  BORA: MONSTRO, 2 ; Da o valor 2 para MONSTRO
  BORA: MONSTRO, MONSTRO * 2 ; Multiplica o valor de MONSTRO por 2
  CE QUER VER ESSA PORRA: "MONSTRO: ", MONSTRO, "IBIRAPUERA: " + IBIRAPUERA
SAINDO DA JAULA
```

## Hello world, cumpade!
```rust
JAULA SHOW
  CE QUER VER ESSA PORRA: "BORA, " + CUMPADE + "!"
SAINDO DA JAULA
```

## Seções e condicionais
```rust
JAULA OUTRO
  CE QUER VER ESSA PORRA: "estou em outra"
SAINDO DA JAULA

JAULA DIFERENTE
  CE QUER VER ESSA PORRA: "deu diferente"

JAULA SHOW
  É HORA DO: OUTRO ; Passa a execução pra OUTRO
  VEM, PORRA: MUTANTE, "FIBRA"
  É ELE QUE A GENTE QUER: MUTANTE, "AGUA COM MUSCULO" ; Compara MUTANTE com "AGUA COM MUSCULO"
  NUM É ELE: DIFERENTE ; Caso seja diferente, execute DIFERENTE
SAINDO DA JAULA
```

## Não quero falar com bandeirantes!
```rust
JAULA BANDEIRANTES
    CE QUER VER ESSA PORRA: "NÃO QUERO FALAR COM BANDEIRANTES"
SAINDO DA JAULA

JAULA OUTRO
    CE QUER VER ESSA PORRA: "COM " + EMISSORA + " EU FALO"
SAINDO DA JAULA

JAULA SHOW
    VEM: EMISSORA ; Cria variavel EMISSORA
    BORA CUMPADE, PORRA: EMISSORA ; Guarda valor da entrada em EMISSORA com letras maiusculas
    E ELE QUE A GENTE QUER: EMISSORA, "BANDEIRANTES" ; Compara com bandeirantes
    E ELE MEMO: BANDEIRANTES ; Caso seja igual, execute bandeirantes
    NUM E ELE: OUTRO ; Diferente? Execute outro
SAINDO DA JAULA
```