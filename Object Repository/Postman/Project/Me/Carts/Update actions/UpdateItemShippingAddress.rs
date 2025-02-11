<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>UpdateItemShippingAddress</name>
   <tag></tag>
   <elementGuidId>d98df128-8fbd-432c-98b9-8b35527bd777</elementGuidId>
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
      <webElementGuid>68a56c83-2335-4326-8929-74ccc678f721</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>26dd7bc6-41d8-456d-b8d5-61c17757ea1f</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.6.9</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${host}/${project-key}/me/carts/${cart-id}</restUrl>
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
      <id>d4cb9f10-1ff2-4be7-8033-d28825b319ec</id>
      <masked>false</masked>
      <name>host</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.project-key</defaultValue>
      <description></description>
      <id>b30b432d-ee01-4eea-8504-51307e88d8df</id>
      <masked>false</masked>
      <name>project-key</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.cart-id</defaultValue>
      <description></description>
      <id>6f4bf66f-2cf0-40b1-b69b-95dd6a000631</id>
      <masked>false</masked>
      <name>cart-id</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.cart-version</defaultValue>
      <description></description>
      <id>f703a3e9-2cd8-4d23-a818-4854982dfb50</id>
      <masked>false</masked>
      <name>cart-version</name>
   </variables>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
