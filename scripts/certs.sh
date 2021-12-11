#!/bin/bash

dir="../docker/certs"
[[ -d ${dir} ]] || mkdir -p ${dir}
cd ${dir}

# generate CA private key
openssl genrsa 2048 > ca-key.pem

# generate CA cert
openssl req -new -x509 -nodes -days 365 \
   -subj "/CN=ca.sys.skyrocket.projectexchange.org/C=DE/L=BW" \
   -key ca-key.pem \
   -out ca-cert.pem

for service in mariadb; do
	for instance in client server; do
		# generate private key
		openssl req -newkey rsa:2048 -nodes -days 365 \
		   -subj "/CN=${service}-${instance}.sys.skyrocket.projectexchange.org/C=DE/L=BW" \
		   -keyout ${instance}-key.pem \
		   -out ${instance}-req.pem

		# generate certificate
		openssl x509 -req -days 365 -set_serial 01 \
		   -in ${instance}-req.pem \
		   -out ${instance}-cert.pem \
		   -CA ca-cert.pem \
		   -CAkey ca-key.pem

		# validate certificate and ca file
		openssl verify -CAfile ca-cert.pem \
		   ca-cert.pem \
		   ${instance}-cert.pem

		# cleanup
		mkdir -p ${service}/${instance}
		rm *-req.pem
		mv ${instance}-*.pem ${service}/${instance}/
	done
done
