# Rust
Rust je programski jezik opšte namene koji je cenjen zbog brzine izvršavanja programa, bezbednosti tipova i bezbednosti memorije. Podržava više programskih paradigmi kao što su funkcionalno programiranje, imperativno programiranje i objektno orijentisano programiranje. Rust se ističe u odnosu na ostale programske jezike po bezbednosti memorije bez upotrebe garbage collector-a, već uz pomoć borrow checker-a koji prati životni ciklus referenci u compile time-u. Rust je 2006. godine kreirao softverski inženjer Graydon Hoar. Rust je široko korišćen u web servisima i sistemskom programiranju. On je statički tipiziran jezik i kompajlira se direktno u mašinski kod što mu omogućava performanse slične C i C++ jezicima. 

## Karakteristike jezika
Rust garantuje sigurnost memorije kroz ownership sistem gde svaka vrednost ima tačno jednog vlasnika i automatski se dealocira kada vlasnik izađe iz scope-a. Borrowing pravila omogućavaju privremeni pristup podacima kroz reference, ali zadatak kompajlera je da proveri da ne može postojati istovremeno mutable i immutable referenca na iste podatke, što eliminiše data race u compile time-u. Rust je statički tipiziran i podržava generičke tipove, trait-ove (slično interface-ima), kao i algebarske tipove podataka kroz enumeracije koje se koriste za pattern matching. Error handling je eksplicitan kroz Result<T, E> enum bez exception mehanizma, što tera programere da obrađuju greške. Rust podržava ,,fearless concurrency" gde ownership i type sistem sprečavaju data races automatski kroz Send i Sync marker trait-ove.

## Arhitektura


## Reference
https://en.wikipedia.org/wiki/Rust_(programming_language)

https://medium.com/codex/rust-101-everything-you-need-to-know-about-rust-f3dd0ae99f4c
