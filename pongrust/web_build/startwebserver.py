#!/usr/bin/env python3
import http.server
import socketserver
import os
import sys

PORT = 8080

class Handler(http.server.SimpleHTTPRequestHandler):
    pass

def main():
    # Assicura che il server rimanga attivo consentendo il riutilizzo rapido dell'indirizzo
    socketserver.TCPServer.allow_reuse_address = True
    
    try:
        with socketserver.TCPServer(("", PORT), Handler) as httpd:
            print(f"🚀 Server locale attivo su: http://localhost:{PORT}")
            print(f"📂 Cartella servita: {os.getcwd()}")
            print("Premi CTRL+C per arrestare il server.")
            httpd.serve_forever()
    except KeyboardInterrupt:
        print("\n\n🛑 Server arrestato correttamente.")
        sys.exit(0)

if __name__ == "__main__":
    main()