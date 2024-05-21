<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>UpdateItemShippingAddress</name>
   <tag></tag>
   <elementGuidId>5eabd6e0-091e-4e5c-8dd9-afc48aaa1a9f</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>true</autoUpdateContent>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n    \&quot;version\&quot;: ${cart-version},\n    \&quot;actions\&quot;: [\n        {\n            \&quot;action\&quot; : \&quot;updateItemShippingAddress\&quot;,\n            \&quot;address\&quot; : {\n              \&quot;key\&quot; : \&quot;exampleKey\&quot;,\n              \&quot;title\&quot; : \&quot;My Address\&quot;,\n              \&quot;salutation\&quot; : \&quot;Mr.\&quot;,\n              \&quot;firstName\&quot; : \&quot;Example\&quot;,\n              \&quot;lastName\&quot; : \&quot;Person\&quot;,\n              \&quot;streetName\&quot; : \&quot;Example Street\&quot;,\n              \&quot;streetNumber\&quot; : \&quot;4711\&quot;,\n              \&quot;additionalStreetInfo\&quot; : \&quot;Backhouse\&quot;,\n              \&quot;postalCode\&quot; : \&quot;80933\&quot;,\n              \&quot;city\&quot; : \&quot;Exemplary City\&quot;,\n              \&quot;region\&quot; : \&quot;Exemplary Region\&quot;,\n              \&quot;state\&quot; : \&quot;Exemplary State\&quot;,\n              \&quot;country\&quot; : \&quot;DE\&quot;,\n              \&quot;company\&quot; : \&quot;My Company Name\&quot;,\n              \&quot;department\&quot; : \&quot;Sales\&quot;,\n              \&quot;building\&quot; : \&quot;Hightower 1\&quot;,\n              \&quot;apartment\&quot; : \&quot;247\&quot;,\n              \&quot;pOBox\&quot; : \&quot;2471\&quot;,\n              \&quot;phone\&quot; : \&quot;+49 89 12345678\&quot;,\n              \&quot;mobile\&quot; : \&quot;+49 171 2345678\&quot;,\n              \&quot;email\&quot; : \&quot;email@example.com\&quot;,\n              \&quot;fax\&quot; : \&quot;+49 89 12345679\&quot;,\n              \&quot;additionalAddressInfo\&quot; : \&quot;no additional Info\&quot;,\n              \&quot;externalId\&quot; : \&quot;Information not needed\&quot;\n            }\n          }\n    ]\n}&quot;,
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
      <webElementGuid>1d3f68ac-05c3-48f6-8edc-3c4827a28d11</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>3af5ee3b-186a-49b7-844a-2428ebeab8ce</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.6.9</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${host}/${project-key}/carts/${cart-id}</restUrl>
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
      <id>47db2c1f-70ae-40cf-849f-60e16e2631dd</id>
      <masked>false</masked>
      <name>host</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.project-key</defaultValue>
      <description></description>
      <id>b7a57d08-be30-49ac-b84c-303c2cb20a8e</id>
      <masked>false</masked>
      <name>project-key</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.cart-id</defaultValue>
      <description></description>
      <id>f1e6270f-9c56-4451-9705-a1dd0b093e24</id>
      <masked>false</masked>
      <name>cart-id</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.cart-version</defaultValue>
      <description></description>
      <id>6e302baa-9043-42f1-ab88-40428be9d724</id>
      <masked>false</masked>
      <name>cart-version</name>
   </variables>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
