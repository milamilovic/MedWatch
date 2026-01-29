# Rust
Rust je programski jezik opšte namene koji je popularan zbog brzine izvršavanja programa, bezbednosti tipova i bezbednosti memorije. Podržava razne programske paradigme odnosno funkcionalno, imperativno i objektno orijentisano programiranje. Rust obezbeđuje bezbednost memorije i to za razliku od drugih programskih jezika bez upotrebe garbage collector-a, već uz pomoć borrow checker-a koji prati životni ciklus referenci na objekte za vreme kompajliranja. Rust je 2006. godine kreirao softverski inženjer Graydon Hoar. Široko je korišćen u web servisima i sistemskom programiranju. Statički je tipiziran i kompajlira se direktno u mašinski čime se postižu performanse slične C-u i C++-u. 

## Karakteristike jezika
Rust garantuje sigurnost memorije kroz ownership sistem, gde svaka vrednost može da ima tačno jednog vlasnika i memorija se automatski dealocira kada njen vlasnik izađe iz opsega. Pravila pozajmljivanja omogućavaju da se kroz reference privremeno pristupi podacima, ali zadatak kompajlera je da proveri da ne postoji istovremeno i promenljiva i nepromenljiva referenca na iste podatkečime se otklanja mogućnost da se desi data race u toku izvršavanja. Rust podržava generičke tipove, trait-ove (koji su poput interface-a) kao i algebarske tipove podataka. Rukovanje greškama se odvija tako što postoji Result<T, E> enumeracija, što tera programere da obrade sve greške.

## Cargo
Cargo je package manager za rust. On preuzima zavisnosti, kompajlira pakete i postavlja ih na registar paketa. Dependency resolution algoritam koristi verzije koje se definišu u Cargo.toml fajlu da bi pronašao kompatibilne verzije svih dependency-ja (uključujući i tranzitivne) i tako generiše Cargo.lock fajl sa tačnim verzijama svih zavisnosti. Prilikom kompajliranja svaki paket koji se naziva i crate mora da se kompajlira pre crate-ova koji zavise od njega. Ako dođe do neke izmene cargo ponovo kompajlira samo crate-ove koji su se promenili ili zavise od promenjenih crate-ova. Fingerprinting mehanizam kreira hash-eve za svaku zavisnost i build konfiguraciju kako bi detektovao kada je potreban rebuild. Podrazumevano cargo povlači zavisnosti sa crates.io gde korisnici mogu sami da postave pakete, ali se i git repozitorijumi, paketi iz fajl sistema i drugi eksterni izvori takođe mogu navesti kao zavisnosti.

## Arhitektura
<br/>
<img src="https://github.com/user-attachments/assets/08f6a2c0-a863-4028-a3c1-132332d0adad" />
<br/><br/>


### Lekser i parser
todo

### HIR
todo

### MIR
todo

### LLVM IR
todo

## Reference
https://en.wikipedia.org/wiki/Rust_(programming_language)

https://medium.com/codex/rust-101-everything-you-need-to-know-about-rust-f3dd0ae99f4c

https://doc.rust-lang.org/cargo/

https://kanishkarj.github.io/rust-internals-mir
