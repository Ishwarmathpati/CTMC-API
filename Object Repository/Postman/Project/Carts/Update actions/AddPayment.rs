<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>AddPayment</name>
   <tag></tag>
   <elementGuidId>3bccf621-032b-478b-9c18-4476cd224152</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>true</autoUpdateContent>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n    \&quot;version\&quot;: ${cart-version},\n    \&quot;actions\&quot;: [\n        {\n            \&quot;action\&quot; : \&quot;addPayment\&quot;,\n            \&quot;payment\&quot; : {\n              \&quot;id\&quot; : \&quot;${payment-id}\&quot;,\n              \&quot;typeId\&quot; : \&quot;payment\&quot;\n            }\n          }\n    ]\n}&quot;,
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
      <webElementGuid>8285450b-1430-4466-8324-2b97de8cb0a8</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>1aed1a73-bca4-497a-8bb3-4810c02b8dad</webElementGuid>
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
      <id>87f4b81a-0a76-4adc-9622-4b2ec4ebc237</id>
      <masked>false</masked>
      <name>host</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.project-key</defaultValue>
      <description></description>
      <id>02cdf13f-1f9d-4fd6-ac69-e1aa454c5dea</id>
      <masked>false</masked>
      <name>project-key</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.cart-id</defaultValue>
      <description></description>
      <id>c00ea7c6-c8ee-41fe-89b7-418bc592f6ae</id>
      <masked>false</masked>
      <name>cart-id</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.cart-version</defaultValue>
      <description></description>
      <id>6d808851-91bc-4897-acea-848234b159ff</id>
      <masked>false</masked>
      <name>cart-version</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.payment-id</defaultValue>
      <description></description>
      <id>d7ed9895-e8b8-40ea-912f-c3143594ed93</id>
      <masked>false</masked>
      <name>payment-id</name>
   </variables>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
