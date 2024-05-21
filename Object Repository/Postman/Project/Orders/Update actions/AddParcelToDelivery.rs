<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>AddParcelToDelivery</name>
   <tag></tag>
   <elementGuidId>12c6e92a-1fdd-4b5f-9c69-bcc8be156cff</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>true</autoUpdateContent>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n    \&quot;version\&quot;: ${order-version},\n    \&quot;actions\&quot;: [\n        {\n            \&quot;action\&quot; : \&quot;addParcelToDelivery\&quot;,\n            \&quot;deliveryId\&quot; : \&quot;${deliveryId}\&quot;,\n            \&quot;measurements\&quot; : {\n              \&quot;heightInMillimeter\&quot; : 11,\n              \&quot;widthInMillimeter\&quot; : 11\n            },\n            \&quot;trackingData\&quot; : {\n              \&quot;trackingId\&quot; : \&quot;${trackingId}\&quot;,\n              \&quot;carrier\&quot; : \&quot;TNT\&quot;,\n              \&quot;provider\&quot; : \&quot;providerName\&quot;,\n              \&quot;providerTransaction\&quot; : \&quot;${transactionId}\&quot;,\n              \&quot;isReturn\&quot; : false\n            },\n            \&quot;items\&quot; : {\n              \&quot;id\&quot; : \&quot;${lineItemId}\&quot;,\n              \&quot;quantity\&quot; : 2\n            }\n          }\n    ]\n}&quot;,
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
      <webElementGuid>f5ec0c82-0132-4a4c-a365-f6e5906e60e8</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>a851c355-ba53-4005-be3c-7ab6b44b6809</webElementGuid>
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
      <id>555a2fce-81a3-48fc-9bb8-cc85bfef068f</id>
      <masked>false</masked>
      <name>host</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.project-key</defaultValue>
      <description></description>
      <id>9f08065e-461a-45c6-b7da-5b53a78a1525</id>
      <masked>false</masked>
      <name>project-key</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.order-id</defaultValue>
      <description></description>
      <id>aee2cade-091c-4c85-acaa-8590ce98146e</id>
      <masked>false</masked>
      <name>order-id</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.order-version</defaultValue>
      <description></description>
      <id>d82a2c62-fc36-4645-8b1d-40c5faa888e3</id>
      <masked>false</masked>
      <name>order-version</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.deliveryId</defaultValue>
      <description></description>
      <id>b4319a3a-426e-4792-a0c4-3fcedcbb524f</id>
      <masked>false</masked>
      <name>deliveryId</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.trackingId</defaultValue>
      <description></description>
      <id>5c25e843-07d2-4f18-874e-8d39eb9c8928</id>
      <masked>false</masked>
      <name>trackingId</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.transactionId</defaultValue>
      <description></description>
      <id>1adb7c11-c7e0-44ca-98f5-4bd47c2cbc99</id>
      <masked>false</masked>
      <name>transactionId</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.lineItemId</defaultValue>
      <description></description>
      <id>f3e0d57e-7a6f-49d7-9b99-670ab55cba98</id>
      <masked>false</masked>
      <name>lineItemId</name>
   </variables>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
