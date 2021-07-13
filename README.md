# Algoritmo del banchiere - Rust

L'algoritmo del banchiere è un algoritmo di deadlock prevention.

Quando un processo richiede una risorsa disponibile, il sistema deve decidere se l'allocazione richiesta porti il sistema in uno **stato sicuro**. Lo stato è **sicuro** se esiste una **sequenza sicura** di esecuzione di tutti i processi. 

La sequenza $<P_1, P_2, ... , P_n>$ è sicura per ogni $P_i$ se le risorse che $P_i$ può ancora richiedere possono essere soddisfatte dalle risorse correntemente disponibili $+$ le risorse possedute da tutti i $Pj$ con $j < i$.

* Se le risorse richieste da $P_i$ non sono immediatamente disponibili, allora $P_i$ deve attendere che $P_j$ sia terminato
* Quando $P_j$ termina, $P_i$ può ottenere le risorse richieste, eseguire e terminare
* Quando $P_i$ termina $P_{i+1}$ può ottenere le risorse ed eseguire

L'algoritmo del banchiere determina se il nuovo stato è sicuro e, in caso affermativo, permette al sistema di accettare la richiesta. In caso contrario, il processo viene sospeso.

Se $C$ è la **Claim matrix**, A è la **Allocation Matrix**, è necessario verificare la matrice $C-A$ e confrontare gli elementi di quest'ultima con l'**Available vector** $V$. Quindi $C_{ij} - A_{ij} \leq V_j, \forall j$

Negli esempi proposti in C++ e Rust, la struttura dati dedita alla gestione dell'algoritmo è implementata con i seguenti campi:
* ```claim```, la matrice $C$
* ```alloc```, la matrice $A$
* ```available```, il vettore $V$
* ```resources```, il vettore delle risorse totali
* ```running```, il vettore dei processi correntemente in esecuzione

Tutti i campi, poiché sono acceduti da più thread, sono protetti da un Mutex, motivo per cui hanno il prefisso ```m_```. È presente inoltre una variabile condition per ogni risorsa, necessaria per mettere in attesa un thread che non può essere mandato in esecuzione poiché porterebbe il sistema a uno stato non sicuro. 

<br>

## Differenze Rust - C++

Il codice C++ e Rust sono mappati quasi 1 a 1, tuttavia in Rust sono presenti una serie di accorgimenti che permettono non solo di facilitare l'implementazione, ma anche di evitare problemi di sicurezza a tempo di compilazione. Di seguito sono elencate alcune delle differenze:
* Il ```Mutex``` in Rust incapsula tutti i campi della struttura, inseriti in ```BankerAlgorithmData```. In questo modo, ogni volta che è necessario accedere a uno dei campi, è necessario esplicitamente ottenere l'acquisizione di un lock. In caso contrario il codice **non compila**. 
* I ```Mutex``` in Rust vengono sbloccati automaticamente quando termina il blocco che li ha acquisiti. In questo modo è sicuro a tempo di compilazione che ogni chiamata che acquisice i lock, li libera prima di ritornare. In C++ l'unlock è sempre esplicito. 
* Quando vengono generati più thread, in Rust è necessario ottenere degli **smart pointer**, gestiti dal wrapper ```Arc```, *Atomic reference counter*. In questo modo è necessario esplicitamente ottenere **uno** smart pointer per thread attraverso ```Arc::clone```. In tal modo, l'accesso alla struttura condivisa avviene in modo sicuro poiché le API di ```Arc``` forniscono per definizione un accesso safe, la cui implementazione è safe da contratto ed è verificabile formalmente. 
