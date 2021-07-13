# Algoritmo del banchiere - Rust

L'algoritmo del banchiere è un algoritmo di deadlock prevention.

Quando un processo richiede una risorsa disponibile, il sistema deve decidere se l'allocazione richiesta porti il sistema in uno **stato sicuro**. Lo stato è **sicuro** se esiste una **sequenza sicura** di esecuzione di tutti i processi. 

La sequenza $<P_1, P_2, ... , P_n>$ è sicura per ogni $P_i$ se le risorse che $P_i$ può ancora richiedere possono essere soddisfatte dalle risorse correntemente disponibili $+$ le risorse possedute da tutti i $Pj$ con $j < i$.

* Se le risorse richieste da $P_i$ non sono immediatamente disponibili, allora $P_i$ deve attendere che $P_j$ sia terminato
* Quando $P_j$ termina, $P_i$ può ottenere le risorse richieste, eseguire e terminare
* Quando $P_i$ termina $P_{i+1}$ può ottenere le risorse ed eseguire

L'algoritmo del banchiere determina se il nuovo stato è sicuro e, in caso affermativo, permette al sistema di accettare la richiesta. In caso contrario, il processo viene sospeso.

Se $C$ è la **Claim matrix**, A è la **Allocation Matrix**, è necessario verificare la matrice $C-A$ e confrontare gli elementi di quest'ultima con l'**Available vector** $V$. Quindi $C_{ij} - A_{ij} \leq V_j, \forall j$