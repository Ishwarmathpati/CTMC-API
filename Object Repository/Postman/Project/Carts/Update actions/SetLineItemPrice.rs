<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>SetLineItemPrice</name>
   <tag></tag>
   <elementGuidId>3a0fd2d7-f30b-4cd4-87c1-7843d8beacef</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>true</autoUpdateContent>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n    \&quot;version\&quot;: ${cart-version},\n    \&quot;actions\&quot;: [\n        {\n            \&quot;action\&quot; : \&quot;setLineItemPrice\&quot;,\n            \&quot;lineItemId\&quot; : \&quot;${lineItemId}\&quot;,\n            \&quot;externalPrice\&quot; : {\n              \&quot;currencyCode\&quot; : \&quot;EUR\&quot;,\n              \&quot;centAmount\&quot; : 4000\n            }\n          }\n    ]\n}&quot;,
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
      <webElementGuid>f90820e9-d37a-4155-863e-d3a46dac5779</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>b4af63c9-6275-4cc2-999b-449c07f72aab</webElementGuid>
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
      <id>24e41e2b-a651-436e-9c41-9e91d4c167ba</id>
      <masked>false</masked>
      <name>host</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.project-key</defaultValue>
      <description></description>
      <id>1221e9cb-f738-46ad-b6a2-0267e0fa444d</id>
      <masked>false</masked>
      <name>project-key</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.cart-id</defaultValue>
      <description></description>
      <id>841a0bd0-c2b9-4f9d-8b17-518476f1a9b7</id>
      <masked>false</masked>
      <name>cart-id</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.cart-version</defaultValue>
      <description></description>
      <id>548b43d8-6d91-42d0-aa2f-2a8ddb24457f</id>
      <masked>false</masked>
      <name>cart-version</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.lineItemId</defaultValue>
      <description></description>
      <id>283f59dd-dd96-4245-965a-14cd9d0bae00</id>
      <masked>false</masked>
      <name>lineItemId</name>
   </variables>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
