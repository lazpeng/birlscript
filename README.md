# BIRLSCRIPT
É a linguagem de scripting dos programadores codebuilders! Dos que não saem de casa
sem codar pra caralho! Se você ta achando que é moleza, ta enganado, aqui não é
café com músculo, aqui se constrói código, porra!

Se você é um programador mutante e se sente apto pro desafio, vá em frente!
Ajude no desenvolvimento do projeto com ideias, código e muita fibra!

Use o interpretador com a flag *-a* ou *--ajuda-o-maluco-ta-doente* para ver uma lista
de opções que podem ser passadas. Use *-e* ou *--ele-que-a-gente-quer* junto com o nome
de um comando para ver mais sobre ele, ou *-t* ou *--tudo-cumpade* para uma lista de comandos.

Builds estarão disponiveis na pasta *bin* a partir da versão 0.x BETA, que estará funcional.
Por enquanto, o interpretador pode ser compilado com a toolset basica de [Rust](https://rust-lang.org/), porém
não tem toda a funcionalidade implementada.

## Versão 0.1.3 PRÉ-ALFA

*Copyleft(ɔ) 2016 Rafael R Nakano. Nenhum direito reservado.*

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

## Hello world, Porra!
´´´
JAULA SHOW
  CE QUER VER ESSA PORRA: "Hello world, Porra!"
SAINDO DA JAULA
´´´

O código acima imprime na tela "Hello world, Porra!" e fecha.
