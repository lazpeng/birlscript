# Documentação
*introdução*
## Comandos
Listados todos os comandos e um resumo do que cada um faz e
detalhes em relação ao comportamento de cada um
### BIRL (Return)
### NUM VAI DÁ NÃO (Quit)
### CE QUER VER (Print)
### CE QUER VER ISSO (PrintLn)
### *sem nome* (PrintDebug)
### VEM (Declare)
### BORA (Set)
### É ELE QUE A GENTE QUER (Compare)
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
### FALA AÍ (GetStringInput)
### FALA UM NÚMERO (GetNumberInput)
### FALA UM INTEIRO (GetIntegerInput)
### MUDA PRA NÚMERO (ConvertToNum)
### MUDA PRA INTEIRO (ConvertToInt)
### MUDA PRA TEXTO (IntoString)
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

* Variável : Nome da variável pra receber o valor do index pra cada iteração.
Se não existir, é declarada pelo comando.
* Valor de início : Expressão que resulta em um número inteiro que é o primeiro index
* Valor final : Expressão que resulta em um inteiro que é o último index - 1 (isso é, o index nunca chega no valor final). Se o valor final for menor que o inicial (o loop é reverso), *stepping* deve ser usado com um valor negativo
* (opcional) *stepping* : Expressão que resulta em um inteiro que é usado como modificador pro index a cada iteração. (Padrão : 1)
## Variáveis padrão
São variáveis disponíveis no escopo global e não podem ser modificadas. O principal motivo de existirem é pra testes e zoeira.

* CUMPADE : Tem o nome de usuário rodando o programa
* UM : Tem o valor 1
## Instruções e a máquina virtual