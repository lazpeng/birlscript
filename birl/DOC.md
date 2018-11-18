# Documentação
**BirlScript 2.0.0-beta**
**Rafael Rodrigues Nakano <lazpeng@gmail.com>**

BirlScript é uma linguagem de tipagem dinâmica e interpretada
(por enquanto não existe uma ABI/bytecode formada e eu não acho que
vale o esforço pra compilar isso), que nasceu de uma piada em 2016.
Na época eu usei o projeto pra aprender mais sobre Rust, e por isso a
versão 1.0 era horrível. Embora tenha sido um grande aprendizado,
não só em relação a Rust mas também em design e implementação de
linguagens de programação (se bem que a única coisa de design que
eu aprendi foi como *não* fazer as coisas), e por isso eu sempre tive
em mente uma versão 2.0, onde eu pudesse empregar tudo o que eu aprendi
enquanto fazendo o original (que eu não implementei por ser muito diferente
da base que já existia, ou seja, precisava ser do zero) e estudando 
material já existente pra incorporar algorítimos e ideias da indústria
à respeito de linguagens de programação. Só que essa última parte eu não fiz.

Por mais que eu não tenha estudado muito a fundo, eu aprendi algumas coisas meio
que indiretamente, e apliquei onde eu pude. O maior problema é que o design antigo
era extremamente ruim, e *cortava caminho* onde podia. Eu não consigo imaginar
BirlScript sendo um C só que com *keywords* engraçadas, e o coração da lang
tava no design antigo, por mais que fosse ruim.

Então, é, abaixo tem um resumo e descrição de comandos e o comportamento
em geral. Contribuições são bem-vindas, principalmente em relação à linguagem em si,
como nomes de comandos e etc. Entrar em contato comigo ou fazer uma pull request
são opções, e não esqueça de testar tudo e alterar essa documentação pra refletir
o novo comportamento se for necessário.
## Comandos
Listados todos os comandos e um resumo do que cada um faz e
detalhes em relação ao comportamento de cada um
### BIRL (Return)
Retorna pra função anterior. Um valor é opcional

Argumentos :
* (opcional) Valor : O valor pra ser retornado pra função anterior. Se nada for passado, é Null
### NUM VAI DÁ NÃO (Quit)
Encerra e execução do programa
### CE QUER VER (Print)
*Printa* zero ou mais valores pra saída padrão, e só.

Argumentos :
* (opcional) Valor : O que é *printado*.
* ... Valores
### CE QUER VER ISSO (PrintLn)
*Printa* zero ou mais valores pra saída padrão, seguido de uma nova linha

Argumentos :
* (opcional) Valor : mesma coisa
### *sem nome* (PrintDebug)
Esse comando só existe pro console interativo, embora *seja* possível "usá-lo"
normalmente, só não faz nada. O que isso faz é basicamente o mesmo que os dois
acima, mas com informação adicional sobre o tipo do valor, e aceita somente um argumento

Argumentos :
* Valor : O valor
### VEM (Declare)
Declara uma variável com um valor inicial, ou, se nada for passado, Null.
Se a variável já existir em um escopo acima, essa é usada até o fim do escopo.
A variável pode ser modificada por outros comandos, ou seja, não é constante.

Argumentos :
* Nome : Nome dado pra variável
* (opcional) Valor : valor inicial
### BORA (Set)
Muda o valor de uma variável, que já foi declarada anteriormente.

Argumentos :
* Nome : nome da variável
* Valor : novo valor
### É ELE QUE A GENTE QUER (Compare)
Compara dois valores dados como argumentos. Se um dos valores for Null, o
resultado é sempre diferente. Se os dois forem Null, é igual

Argumentos :
* Valor 1
* Valor 2
### FIM (EndSubScope)
Encerra um bloco (condicional ou loop)
### É ELE MEMO (ExecuteIfEqual)
Executa o bloco de comandos se a última comparação foi Igual
### NUM É ELE (ExecuteIfNotEqual)
Executa o bloco de comandos se a última comparação não foi Igual
### MENOR OU É MEMO (ExecuteIfEqualOrLess)
Executa o bloco de comandos se a última comparação foi Igual ou Menor
### É MENOR (ExecuteIfLess)
Executa o bloco de comandos se a última comparação foi Menor
### MAIOR OU É MEMO (ExecuteIfEqualOrGreater)
Executa o bloco de comandos se a última comparação foi Igual ou Maior
### É MAIOR (ExecuteIfGreater)
Executa o bloco de comandos se a última comparação foi Maior
### É HORA DO (Call)
Executa uma função com os argumentos passados. O valor de retorno da função
fica na variável TREZE

Argumentos :
* Função
* (opcional) Argumentos ... : Caso tenha
### FALA AÍ (GetStringInput)
Pede um Texto como *input* da entrada padrão.

Argumentos :
* Variável : variável que recebe o valor recebido
### FALA UM NÚMERO (GetNumberInput)
Pede um Número como *input* da entrada padrão.

Argumentos :
* Variável : variável que recebe o valor recebido
### FALA UM INTEIRO (GetIntegerInput)
Pede um Inteiro como *input* da entrada padrão.

Argumentos :
* Variável : variável que recebe o valor recebido
### MUDA PRA NÚMERO (ConvertToNum)
Converte o valor de uma variável pra Número. O valor original é perdido
e a variável recebe o novo valor.

Argumentos :
* Valor : O valor pra ser convertido
### MUDA PRA INTEIRO (ConvertToInt)
Converte o valor de uma variável pra Inteiro. O valor original é perdido
e a variável recebe o novo valor.

Argumentos :
* Valor : O valor pra ser convertido
### MUDA PRA TEXTO (IntoString)
Converte o valor de uma variável pra Texto. O valor original é perdido
e a variável recebe o novo valor.

Argumentos :
* Valor : O valor pra ser convertido
### ENQUANTO É ELE MEMO (ExecuteWhileEqual)
Executa o bloco de comandos enquanto a última comparação for Igual
### ENQUANTO NUM É ELE (ExecuteWhileNotEqual)
Executa o bloco de comandos enquanto a última comparação não for Igual
### ENQUANTO MENOR OU É MEMO (ExecuteWhileEqualOrLess)
Executa o bloco de comandos enquanto a última comparação for Igual ou Menor
### ENQUANTO É MENOR (ExecuteWhileLess)
Executa o bloco de comandos enquanto a última comparação for Menor
### ENQUANTO MAIOR OU É MEMO (ExecuteWhileEqualOrGreater)
Executa o bloco de comandos enquanto a última comparação for Igual ou Maior
### ENQUANTO É MAIOR (ExecuteWhileGreater)
Executa o bloco de comandos enquanto a última comparação for Maior
### REPETE (RangeLoop)
Repete um bloco de comandos por um número de vezes.

Argumentos:
* Variável : Nome da variável pra receber o valor do index pra cada iteração.
Se não existir, é declarada pelo comando.
* Valor de início : Expressão que resulta em um número inteiro que é o primeiro index
* Valor final : Expressão que resulta em um inteiro que é o último index - 1 (isso é, o index nunca chega no valor final). Se o valor final for menor que o inicial (o loop é reverso), *stepping* deve ser usado com um valor negativo
* (opcional) *stepping* : Expressão que resulta em um inteiro que é usado como modificador pro index a cada iteração. (Padrão : 1)
### FAZ UMA LISTA (MakeNewList)
Cria uma nova lista vazia. Se a variável passada como argumento já existir, o valor nela é perdido
e substituído pela lista. Se não, ela é criada

Argumentos :
* Nome : nome pra lista
### FALA O TAMANHO (QueryListSize)
Busca pelo tamanho de uma lista, e coloca esse valor na segunda variável passada como argumento

Argumentos :
* Lista : Nome da variável carregando a lista
* Tamanho : Nome da variável pra receber o tamanho
### PÕE ISSO AQUI (AddListElement)
Adiciona um elemento em uma lista. Opcionalmente, você pode escolher em que posição colocar o elemento.
Se a posição especificada for depois do fim da lista, o elemento é colocado no final

Argumentos :
* Lista : Nome da lista
* Variável : Elemento pra ser adicionado
* (opcional) Índice : Aonde colocar o elemento
### TIRA ESSE (RemoveListElement)
Remove um elemento do índice passado. Se o índice for inválido, um erro acontece.

Argumentos :
* Lista
* Índice do elemento
### ME DÁ ESSE (IndexList)
Indexa um elemento da lista passada e coloca o valor dele na variável passada

Argumentos :
* Lista
* Índice : De onde tirar o elemento
* Elemento : Variável pra receber o valor do elemento
## Variáveis padrão
São variáveis disponíveis no escopo global e não podem ser modificadas. O principal motivo de existirem é pra testes e zoeira.

* CUMPADE : Tem o nome de usuário rodando o programa
* UM : Tem o valor 1
## Instruções e a máquina virtual