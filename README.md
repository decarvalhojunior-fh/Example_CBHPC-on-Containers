# PARALLEL VERSION (Rust + MPI): non-containerized / monolithic

## Como executar

```
mpirun -np N <path to executable file>
```
, onde N é um número par. 

Nosso exemplo inicial usará N=4. Caso esteja executando de dentro do projeto Rust, ```<path to executable file>``` pode ser ```cargo run```.

Com a finalidade de explicar como opera o MPI, se o meu cluster é formado por quatro nós com IPs (fictícios) 1.1.1.1, 2.2.2.2, 3.3.3.3, 4.4.4.4, eu posso criar um arquivo de _hosts_ (_host file_), denominado _hosts.config_, com o seguinte conteúdo:
```
1.1.1.1
2.2.2.2
3.3.3.3
4.4.4.4
```
Então, posso executar o programa com a seguinte linha de comando, a partir de algum nó do cluster ao qual tais IPs sejam visíveis:
```
mpirun --map-by node --hostfile hosts.config <path to executable file>
```

O ```<path to executable file>``` deve apontar para o mesmo local em todos os nós. 

O flag ```--map-by node``` estabelece que os processos serão distribuídos entre os nós descritos em _hosts.config_ usando a estratégia _round-robin_. No caso, se N=4, cada nó recebe exatamente 1 processo. Entenda a semẫntica do ```--map-by``` clicando [aqui](https://blog.mylab.cc/2021/02/05/Understanding-MPI-map-by-and-bind-to-option).

Portanto, na execução no K8s, creio que seria possível, por exemplo, existir um pod mestre (_master_), executando em qualquer nó, além dos 4 pods trabalhadores (_workers_) que executariam o programa MPI, que conhecesse os IPs desses 4 pods trabalhadores e executasse o comando ```mpirun``` para lançar programas  MPI sobre eles. 
