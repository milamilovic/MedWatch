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
Lekser odnosno tokenizator izvodi prvu fazu kompajliranja. On čita source kod kao običan tekst i pretvara ga u tokene koji su najmanje jedinice jezika i mogu biti ključne reči, identifikatori, literali ili operatori. Rust lekser je implementiran kao biblioteka *rustc_lexer* i to tako da radi bez alokacije memorije i zbog toga je veoma brz. Naredni korak u kompajliranju je parsiranje. Parser uzima niz tokena koji lekser kreira i gradi AST (Abstract Syntax Tree). Rust parser je implementiran tako da može da se oporavi od grešaka što je korisno zbog toga što može da prikaže korisniku sve sintaksne greške odjednom umesto da stane nakon prve. AST čuva i informacije o poziciji što omogućava preciznije poruke o greški koje korisniku pokazuju tačno gde se greška nalazi. 

### HIR
HIR odnosno High-level Intermediate Representation nastaje nakon AST-a gde je sintaksa pojednostavljena odnosno prilagođenija kompajleru, ali još uvek ljudski čitljiva. Proces koji se zove HIR lowering pretvara AST u HIR strukturu se nazizva AST lowering. HIR se koristi i za type inference odnosno automatsku detekciju tipova, ali i type checking gde se tip koji je napisan u programu poredi sa onim koji je kompajler detektovao.   

### MIR
HIR se dodatno spušta na MIR odnosno Mid-level Intermediate Representation koji se koristi za borrow checking uz pomoć THIR strukture koja je dodatno pojednostavljeni HIR. Borrow checker zapravo radi analizu toka podataka i proverava da li su pravila za pozajmlijvanje ispoštovana u svim mogućim scenarijima. Većina optimizacija koda se dešava na MIR-u. MIR lowering-om se pattern matching iz HIR-a pretvara u decision stabla, petlje pretvaraju u  control flow graph, a složeni izrazi se razlažu na jednostavnije operacije. Ovde se takođe odvija memorphization koji pretvara generički kod u konkretne verzije za svaki tip koji se koristi. Na taj način veličina binarnog koda bude veća ali se ubrzava izvršavanje. 

### LLVM IR
LLVM je set kompajlerskih tehnologija koji kreira optimizovan binarni kod za konkretnu platformu. *rustic* koristi LLVM za generianje koda tako da MIR mora prvo da se pretvori u LLVM IR (language-independent intermediate representation) koji je poput asemblera na visokom niovu koji sadrži mnogo anotacija. Taj LLVM IR se prosleđuje LLVM-u koji radi još optimizacija i proizvodi mašinski kod. 

## Reference
https://en.wikipedia.org/wiki/Rust_(programming_language)

https://medium.com/codex/rust-101-everything-you-need-to-know-about-rust-f3dd0ae99f4c

https://doc.rust-lang.org/cargo/

https://kanishkarj.github.io/rust-internals-mir

https://rustc-dev-guide.rust-lang.org/overview.html

https://doc.rust-lang.org/nightly/nightly-rustc/rustc_lexer/index.html

https://en.wikipedia.org/wiki/LLVM
