VAPID
-----

VAPID authentication prevents unknown sources sending notifications to the client and is required by all current
browsers when sending a payload.

The private key to be used by the server can be generated with OpenSSL:

```
openssl ecparam -genkey -name prime256v1 -out private_key.pem
```

To derive a public key from the just-generated private key, to be used in the JavaScript client:

```
openssl ec -in private_key.pem -pubout -outform DER|tail -c 65|base64|tr '/+' '_-'|tr -d '\n' > ./public/public.key
```
