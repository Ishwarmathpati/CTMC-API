<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>SetBillingAddress</name>
   <tag></tag>
   <elementGuidId>2db18a55-10aa-409b-9606-83a2c9d7c5ee</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>true</autoUpdateContent>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n    \&quot;version\&quot;: ${order-version},\n    \&quot;actions\&quot;: [\n        {\n            \&quot;action\&quot; : \&quot;setBillingAddress\&quot;,\n            \&quot;address\&quot; : {\n              \&quot;key\&quot; : \&quot;exampleKey\&quot;,\n              \&quot;title\&quot; : \&quot;My Address\&quot;,\n              \&quot;salutation\&quot; : \&quot;Mr.\&quot;,\n              \&quot;firstName\&quot; : \&quot;Example\&quot;,\n              \&quot;lastName\&quot; : \&quot;Person\&quot;,\n              \&quot;streetName\&quot; : \&quot;Example Street\&quot;,\n              \&quot;streetNumber\&quot; : \&quot;4711\&quot;,\n              \&quot;additionalStreetInfo\&quot; : \&quot;Backhouse\&quot;,\n              \&quot;postalCode\&quot; : \&quot;80933\&quot;,\n              \&quot;city\&quot; : \&quot;Exemplary City\&quot;,\n              \&quot;region\&quot; : \&quot;Exemplary Region\&quot;,\n              \&quot;state\&quot; : \&quot;Exemplary State\&quot;,\n              \&quot;country\&quot; : \&quot;DE\&quot;,\n              \&quot;company\&quot; : \&quot;My Company Name\&quot;,\n              \&quot;department\&quot; : \&quot;Sales\&quot;,\n              \&quot;building\&quot; : \&quot;Hightower 1\&quot;,\n              \&quot;apartment\&quot; : \&quot;247\&quot;,\n              \&quot;pOBox\&quot; : \&quot;2471\&quot;,\n              \&quot;phone\&quot; : \&quot;+49 89 12345678\&quot;,\n              \&quot;mobile\&quot; : \&quot;+49 171 2345678\&quot;,\n              \&quot;email\&quot; : \&quot;email@example.com\&quot;,\n              \&quot;fax\&quot; : \&quot;+49 89 12345679\&quot;,\n              \&quot;additionalAddressInfo\&quot; : \&quot;no additional Info\&quot;,\n              \&quot;externalId\&quot; : \&quot;Information not needed\&quot;\n            }\n          }\n    ]\n}&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Bearer {{ctp_access_token}}</value>
      <webElementGuid>595b2e3d-1710-4a86-a301-6c01534a0ae7</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>9560e6f1-d3c2-4aa2-add6-bdd87d3b11fb</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.6.9</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${host}/${project-key}/orders/${order-id}</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.host</defaultValue>
      <description></description>
      <id>cbf28b1b-8c38-4b55-977c-ff1216b7314d</id>
      <masked>false</masked>
      <name>host</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.project-key</defaultValue>
      <description></description>
      <id>89bd8db4-5bfb-442d-9085-db32a415ff2b</id>
      <masked>false</masked>
      <name>project-key</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.order-id</defaultValue>
      <description></description>
      <id>8d4be4ee-633a-4d23-b99d-c25382cc3414</id>
      <masked>false</masked>
      <name>order-id</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.order-version</defaultValue>
      <description></description>
      <id>a83f8bc3-9aaf-4b55-b775-364e30ca7017</id>
      <masked>false</masked>
      <name>order-version</name>
   </variables>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
