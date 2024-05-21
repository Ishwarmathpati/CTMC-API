<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>SetBillingAddress</name>
   <tag></tag>
   <elementGuidId>c6dfb497-94a9-4201-8a3b-6117c5bdd5c0</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>true</autoUpdateContent>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n    \&quot;version\&quot;: ${cart-version},\n    \&quot;actions\&quot;: [\n        {\n            \&quot;action\&quot; : \&quot;setBillingAddress\&quot;,\n            \&quot;address\&quot; : {\n              \&quot;key\&quot; : \&quot;exampleKey\&quot;,\n              \&quot;title\&quot; : \&quot;My Address\&quot;,\n              \&quot;salutation\&quot; : \&quot;Mr.\&quot;,\n              \&quot;firstName\&quot; : \&quot;Example\&quot;,\n              \&quot;lastName\&quot; : \&quot;Person\&quot;,\n              \&quot;streetName\&quot; : \&quot;Example Street\&quot;,\n              \&quot;streetNumber\&quot; : \&quot;4711\&quot;,\n              \&quot;additionalStreetInfo\&quot; : \&quot;Backhouse\&quot;,\n              \&quot;postalCode\&quot; : \&quot;80933\&quot;,\n              \&quot;city\&quot; : \&quot;Exemplary City\&quot;,\n              \&quot;region\&quot; : \&quot;Exemplary Region\&quot;,\n              \&quot;state\&quot; : \&quot;Exemplary State\&quot;,\n              \&quot;country\&quot; : \&quot;DE\&quot;,\n              \&quot;company\&quot; : \&quot;My Company Name\&quot;,\n              \&quot;department\&quot; : \&quot;Sales\&quot;,\n              \&quot;building\&quot; : \&quot;Hightower 1\&quot;,\n              \&quot;apartment\&quot; : \&quot;247\&quot;,\n              \&quot;pOBox\&quot; : \&quot;2471\&quot;,\n              \&quot;phone\&quot; : \&quot;+49 89 12345678\&quot;,\n              \&quot;mobile\&quot; : \&quot;+49 171 2345678\&quot;,\n              \&quot;email\&quot; : \&quot;email@example.com\&quot;,\n              \&quot;fax\&quot; : \&quot;+49 89 12345679\&quot;,\n              \&quot;additionalAddressInfo\&quot; : \&quot;no additional Info\&quot;,\n              \&quot;externalId\&quot; : \&quot;Information not needed\&quot;\n            }\n          }\n    ]\n}&quot;,
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
      <webElementGuid>7a73fc2d-6269-433f-9d77-4153886057fb</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>9082b6ee-41ee-41f0-8646-17d2e23d6c5a</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.6.9</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${host}/${project-key}/as-associate/${associate-id}/in-business-unit/key=${business-unit-key}/carts/${cart-id}</restUrl>
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
      <id>1a358db5-abb3-4af3-a623-53629024d519</id>
      <masked>false</masked>
      <name>host</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.project-key</defaultValue>
      <description></description>
      <id>09a1e3f2-e76d-4e4d-b313-a594502511c7</id>
      <masked>false</masked>
      <name>project-key</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.associate-id</defaultValue>
      <description></description>
      <id>ea3160bb-21ef-4270-a9ab-e4862c56b09a</id>
      <masked>false</masked>
      <name>associate-id</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.business-unit-key</defaultValue>
      <description></description>
      <id>1a316c4b-1346-4dbf-9a74-2c0cdfe78fed</id>
      <masked>false</masked>
      <name>business-unit-key</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.cart-id</defaultValue>
      <description></description>
      <id>7eb7415a-011a-4d88-b74a-b567d856041f</id>
      <masked>false</masked>
      <name>cart-id</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.cart-version</defaultValue>
      <description></description>
      <id>27f44509-650f-4fee-aa18-5ec49fb798eb</id>
      <masked>false</masked>
      <name>cart-version</name>
   </variables>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
