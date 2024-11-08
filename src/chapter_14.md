# Chapter 14 Cryptography


## Cryptography Overview

### Symmetric Encryption

- Concept: In symmetric encryption, the same key is used to both encrypt and decrypt data.
- How it works: When a sender wants to send an encrypted message, they use a shared secret key to encrypt the data. The receiver, who also has the same key, can decrypt it.
- Key Management: Since the same key is used for both encryption and decryption, the key must be kept secret and securely shared between the sender and receiver. If someone intercepts the key, they can access all encrypted data.
- Use cases: Symmetric encryption is efficient and fast, making it ideal for encrypting large amounts of data, like in AES for file encryption or VPNs.
- Examples: AES (Advanced Encryption Standard), DES (Data Encryption Standard), and 3DES (Triple DES).

### Asymmetric Encryption

- Concept: In asymmetric encryption, two different but mathematically linked keys are used—a public key and a private key.
- How it works: The public key is used to encrypt data, and the private key is used to decrypt it. The public key can be shared openly, but the private key remains confidential to the owner.
- Encryption: When someone wants to send a secure message, they encrypt it using the recipient’s public key. Only the recipient, with their private key, can decrypt it.
- Digital Signatures: Asymmetric encryption can also be used for signing data. The sender signs the data using their private key, and anyone with the sender’s public key can verify the signature, confirming the sender’s identity and data integrity.
- Key Management: Since the public key can be openly shared, it’s easier to manage than symmetric encryption for secure key sharing. The private key, however, must be kept secure.
- Use cases: Asymmetric encryption is slower and more computationally intense, so it’s typically used for secure key exchange, digital signatures, and smaller data exchanges.
- Examples: RSA (Rivest-Shamir-Adleman), ECC (Elliptic Curve Cryptography), and DSA (Digital Signature Algorithm).

### Key Differences

- Keys: Symmetric encryption uses one key, while asymmetric encryption uses a key pair (public and private).
- Speed: Symmetric encryption is generally faster and suitable for large data. Asymmetric encryption is slower but more secure for key exchanges and identity verification.
- Use in Practice: Often, symmetric encryption is used to encrypt the actual data, and asymmetric encryption is used to securely exchange the symmetric key (for instance, in SSL/TLS for secure web connections).

In practice, many systems combine both methods, taking advantage of symmetric encryption’s speed and asymmetric encryption’s security for an optimal balance.



## What is a Hash?

A hash is the result of a mathematical function (called a hash function) that transforms an input (or “message”) of any length into a fixed-length string of characters. This output is known as the hash value, digest, or checksum.

Properties of a Hash Function

A good cryptographic hash function has the following properties:
1. Fixed Length: No matter the size of the input data, the output hash has a fixed length. For example, SHA-256 always produces a 256-bit (32-byte) hash value.
2. Deterministic: The same input will always produce the same hash value.
3. Efficient: It is quick to compute the hash value for any given input.
4. Irreversibility (One-Way Function): It is computationally infeasible to reverse the process and get the original input from the hash value.
5. Avalanche Effect: A small change in the input (even changing a single bit) will produce a significantly different hash value.
6. Collision Resistance: It is extremely unlikely that two different inputs will produce the same hash value.

How Hashes Are Used

Hashes are widely used in computer science and cryptography. Here are some common applications:
1. Data Integrity: Hashes are used to verify that data has not been altered. When you download a file, for example, the file may come with a hash value. By running the same hash function on the file you downloaded, you can compare the two hashes. If they match, the file is unaltered.
2. Password Storage: Instead of storing passwords directly, systems store the hash of the password. When you log in, the system hashes the password you enter and compares it to the stored hash. This way, even if the password database is compromised, the original passwords are not immediately exposed.
3. Digital Signatures: Hashes are used to create a “fingerprint” of data for digital signing, ensuring that the data has not been tampered with.
4. Hashing Algorithms: Popular hashing algorithms include MD5, SHA-1, SHA-256, and SHA-3. Note that some, like MD5 and SHA-1, are considered insecure because vulnerabilities have been discovered that make it easier to find collisions.

Example

For instance, using the SHA-256 hash function:
- Input: "hello"
- Hash Value: 2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824

If you change the input even slightly, such as "Hello" (note the uppercase ‘H’), the hash value will be completely different:
- Input: "Hello"
- Hash Value: 185f8db32271fe25f561a6fc938b2e264306ec304eda518007d1764826381969

This demonstrates the avalanche effect, where even a minor change in the input results in a drastically different hash.

Summary

A hash is a fixed-length output derived from input data of any size. It is used to ensure data integrity, securely store passwords, and support various cryptographic operations. The key features of a hash function—deterministic, one-way, collision-resistant, and sensitive to input changes—make it a crucial component of modern security systems.

## What is fingerprint?

A **fingerprint** in cryptography refers to a unique and compact representation of data. It is typically generated by applying a cryptographic hash function (like SHA-256) to a larger piece of data, such as a public key or a digital certificate. The resulting "fingerprint" is a short and unique string that acts as a summary of the original data.

### Uses of Fingerprints
1. **Identity Verification**: In security systems, fingerprints are used to verify the authenticity of a public key or certificate. When you download a public key, for instance, you can check its fingerprint to ensure that the key hasn’t been tampered with.
2. **Certificate Inspection**: When visiting a secure website (HTTPS), your browser checks the fingerprint of the website’s certificate to verify its legitimacy and make sure it was issued by a trusted Certificate Authority (CA).
3. **Software Integrity**: Fingerprints can be used to confirm that software or files haven't been modified. When downloading software, you may see a hash value that you can use to check against the fingerprint of the downloaded file.

### Example
If a public key is hashed using SHA-256, the output is a unique string like `3b:d7:ba:7e:52:1f:8c:6e:34:8f:76:e9:2f:2b:ea:ab`. This string is the **fingerprint** of the public key and can be used to verify that the public key is authentic.

In essence, a fingerprint serves as a reliable, concise way to identify and verify the integrity and authenticity of data in various cryptographic applications.

## What is salt?

In cryptography, a **salt** is a random value added to data (typically a password) before hashing it. The purpose of a salt is to enhance security by ensuring that even if two users have the same password, their hashed values will be different.

### How Salt Works
1. When a password is created or stored, a random salt is generated and combined with the password.
2. The combined value (password + salt) is then hashed using a cryptographic hash function, like SHA-256.
3. The resulting hash and the salt are both stored in the database.

### Why Use Salt?
1. **Prevent Hash Collisions**: Without a salt, two identical passwords would produce the same hash, making it easier for attackers to recognize and exploit common passwords. A salt ensures that even identical passwords have different hashes.
2. **Defend Against Rainbow Table Attacks**: A rainbow table is a precomputed table of hashes for common passwords, used to crack hashed passwords quickly. By adding a unique salt to each password, it becomes impractical to use rainbow tables, as each password hash is unique and would require a different table.
3. **Increase Security**: Salts make it more difficult for attackers to use precomputed hash databases or brute-force methods to crack passwords, as they need to recompute the hashes for each password attempt using the specific salt.

### Example
1. Suppose a password is "mypassword123".
2. A random salt, say "xy7d8z9a", is generated.
3. The password and salt are combined: "mypassword123xy7d8z9a".
4. The combined value is hashed, resulting in a unique hash.

When a user logs in, the system uses the stored salt to hash the entered password and compares it to the stored hash. If they match, the password is verified.

### Important Note
- **Salts should be unique for each user**: Using the same salt for every user weakens security.
- **Salts should be random and sufficiently long**: This makes it harder for attackers to guess or use brute-force techniques.

In summary, a salt enhances the security of hashed passwords by adding an extra layer of complexity, making it harder for attackers to crack them.


## What is Digital Sign?

Digital signing is a process used to verify the authenticity and integrity of a message, document, or piece of data. It ensures that:
1. The data has not been altered since it was signed (integrity).
2. The data is genuinely from the person or entity that claims to have signed it (authenticity).

How Signing Works

Digital signing involves two main components: a private key and a public key, which are part of an asymmetric encryption system. Here’s a step-by-step explanation:
1. Hashing the Data:
   • First, a mathematical function called a hash function (e.g., SHA-256) is used to create a unique fixed-size hash of the original data.
   • The hash acts as a “fingerprint” of the data. If even one bit of the data changes, the hash will be completely different.
2. Encrypting the Hash (Creating the Signature):
   • The data’s hash is then encrypted using the sender’s private key. This encrypted hash is called the digital signature.
   • Only the sender’s private key can create this signature, which links the signature specifically to the sender.
3. Attaching the Signature:
   • The digital signature is attached to the original data, and both are sent to the recipient.

How Verification Works

When the recipient receives the signed data:
1. Recreating the Hash:
   • The recipient uses the same hash function to create a new hash from the received data.
2. Decrypting the Signature:
   • The recipient uses the sender’s public key to decrypt the digital signature and retrieve the original hash created by the sender.
3. Comparing Hashes:
   • The recipient compares the newly generated hash with the hash from the decrypted signature.
   • If the two hashes match:
   • The data has not been altered (integrity).
   • The signature was created with the sender’s private key, proving that the data is genuinely from the sender (authenticity).
   • If the hashes do not match, either the data was tampered with, or the signature is not valid.

Use Cases

• Document Signing: To verify the author and prevent tampering of digital documents.
• Software Distribution: To ensure that software has not been altered and is from a trusted source.
• Email Security: To verify the sender and ensure the content hasn’t been changed.

Summary

Digital signing uses cryptographic techniques to ensure data integrity and authenticity. The private key creates the signature, while the public key allows others to verify it. This process helps secure sensitive communications, protect data, and authenticate identities.

## What is a Certificate?

A certificate, in the context of digital security, is a type of digital document used to verify the identity of a person, device, or service and to enable secure communication. Certificates are a crucial component of Public Key Infrastructure (PKI) and are used in various security protocols, such as HTTPS, to establish trust on the internet.

What Does a Certificate Contain?

A digital certificate typically includes:
1. Subject: Information about the identity of the certificate holder, such as the name of a person, organization, or domain (e.g., www.example.com).
2. Public Key: The public key associated with the certificate holder. This key is used for encryption and digital signature verification.
3. Issuer: The Certificate Authority (CA) that issued the certificate. The CA vouches for the identity of the certificate holder.
4. Validity Period: The start and expiration dates of the certificate. It is only considered valid within this timeframe.
5. Serial Number: A unique number assigned by the CA to identify the certificate.
6. Signature: A digital signature from the CA, which ensures that the certificate has not been tampered with and is authentic.
7. Extensions: Additional information or attributes that specify how the certificate should be used (e.g., for server authentication, code signing, etc.).

Purpose of a Certificate

1. Authentication: Certificates verify the identity of the certificate holder. For example, when you visit a website with HTTPS, the website’s certificate assures you that the site is genuine and not a malicious imposter.
2. Encryption: Certificates provide the public key used to encrypt data. When you send information over a secure connection (e.g., to a bank website), the data is encrypted using the website’s public key.
3. Data Integrity: Certificates use digital signatures to ensure that the data exchanged has not been altered in transit.

How Certificates Work

Certificates are issued and managed by trusted entities called Certificate Authorities (CAs). Here’s a simplified overview of how they are used in secure communications:
1. Issuance: When a website or entity wants to prove its identity, it requests a certificate from a CA. The CA verifies the entity’s information and issues a certificate containing the entity’s public key.
2. Trust Establishment: The CA’s own certificate (which is trusted by default in browsers and operating systems) vouches for the authenticity of the issued certificate.
3. Secure Communication: When a user connects to a secure website, the website presents its certificate. The user’s browser checks that the certificate is valid, issued by a trusted CA, and hasn’t expired or been revoked. If everything checks out, the browser establishes a secure, encrypted connection.

Types of Certificates

1. SSL/TLS Certificates: Used to secure websites and enable HTTPS. They ensure encrypted communication between a web server and a browser.
2. Code Signing Certificates: Used by software developers to sign code or software, proving that the software is authentic and hasn’t been tampered with.
3. Email Certificates: Used to sign and encrypt emails, ensuring secure and authentic email communication.
4. Client Certificates: Used to authenticate users or devices in secure systems.

Summary

A digital certificate is a secure digital document that uses cryptographic techniques to establish and verify the identity of an entity and to facilitate secure communication. It is issued by a trusted Certificate Authority and contains critical information, including the public key of the entity it identifies, making it a fundamental component of internet security.


Sure, here’s an explanation of the chain of certificates:

## What is a Chain of Certificates?

A chain of certificates is a series of digital certificates linked together to establish trust between an end entity (like a website) and a root Certificate Authority (CA). It’s also called a certificate chain or trust chain. This chain ensures that when you encounter a certificate, you can trust it because it is backed by a known, trusted authority.

How the Chain of Certificates Works

In a certificate chain, each certificate in the chain is signed by the one above it. Here’s how it’s structured:
1. Root Certificate (Root CA):
   - At the top of the chain is the root certificate, which is issued by a trusted Certificate Authority (CA). Root CAs are well-known and are pre-installed in operating systems and browsers as trusted authorities.
   - The root certificate is self-signed, meaning it is verified by itself. Operating systems and browsers inherently trust these root certificates because they come from reputable authorities.
2. Intermediate Certificates (Intermediate CA):
   - Between the root and the end-entity certificate, there may be one or more intermediate certificates. These are issued by the root CA or other intermediates and act as “delegates” of the root.
   - Intermediate certificates add a layer of security by keeping the root certificate offline. They allow the root CA to delegate signing responsibilities without directly exposing its private key.
3. End-Entity Certificate (Leaf Certificate):
   - The end-entity certificate is issued by an intermediate CA and is the certificate directly associated with the server or application (like a website).
   - This certificate proves the identity of the server or application to users. It’s what browsers and clients see when they connect to a secure website.

Example of a Certificate Chain in Use

Let’s take a typical example of a secure website using HTTPS:
1. Website Certificate (End-Entity Certificate): A website’s certificate is presented to your browser when you connect. It is issued by an intermediate CA.
2. Intermediate Certificate:  The website’s certificate is signed by an intermediate CA, which acts as a middle layer of trust.
3. Root Certificate: The intermediate certificate is, in turn, signed by a root CA certificate, which is inherently trusted by the operating system or browser.

When you connect to the website, your browser verifies each certificate in the chain, working up from the end-entity certificate to the root. If each link is valid and trustworthy, the entire chain is trusted, and your browser can establish a secure connection.

Why Use a Chain of Certificates?

The chain of certificates serves several purposes:
1. Scalability: Root CAs issue a limited number of root certificates. Intermediate certificates allow root CAs to scale by issuing intermediate certificates that, in turn, issue many end-entity certificates.
2. Security: By delegating signing to intermediate CAs, root CAs can keep their private keys offline, reducing the risk of compromise. If an intermediate certificate is compromised, the root CA can revoke it without affecting the entire root.
3. Trustworthiness: It establishes a hierarchy of trust. The root certificate is trusted, and this trust is extended through intermediates down to end-entity certificates. This ensures that users can trust the identity and integrity of entities presenting certificates.

Summary

The chain of certificates is a structured hierarchy that allows for secure, trusted connections on the internet. It works by linking a trusted root CA to an end-entity certificate through one or more intermediate certificates. This chain ensures that when you connect to a website or service, you can trust its identity based on the endorsements by recognized authorities.



## AES Encryption
// TODO: example of code
## SHA-256 Hashing
// TODO: example of code

## Signing and Verifying Data
// TODO: example of code

## TLS/SSL Encryption
// TODO: example of code
