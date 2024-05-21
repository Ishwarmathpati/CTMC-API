<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>AddParcelToDelivery</name>
   <tag></tag>
   <elementGuidId>d895c3e3-3bff-41d8-8592-a0c18bb1c51d</elementGuidId>
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
      <webElementGuid>49a7a592-c1dd-463f-a21d-5540fbd9c993</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>a71a3ace-eb9f-48cf-93b3-26677755e030</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.6.9</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${host}/${project-key}/in-store/key=${store-key}/orders/${order-id}</restUrl>
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
      <id>394960bb-5cce-4eb1-b916-d074ade3dc2d</id>
      <masked>false</masked>
      <name>host</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.project-key</defaultValue>
      <description></description>
      <id>e9c35c02-50aa-4631-8e3c-3157d0aaa398</id>
      <masked>false</masked>
      <name>project-key</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.store-key</defaultValue>
      <description></description>
      <id>6fe0d2d0-9f90-41c2-8efd-68ab26486059</id>
      <masked>false</masked>
      <name>store-key</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.order-id</defaultValue>
      <description></description>
      <id>6e9f832c-0487-4150-9162-0676f4097587</id>
      <masked>false</masked>
      <name>order-id</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.order-version</defaultValue>
      <description></description>
      <id>f275de50-0f54-4996-b758-2ed1a113ac18</id>
      <masked>false</masked>
      <name>order-version</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.deliveryId</defaultValue>
      <description></description>
      <id>c5f8bd0f-7eec-436d-a4ca-7581955dbe62</id>
      <masked>false</masked>
      <name>deliveryId</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.trackingId</defaultValue>
      <description></description>
      <id>eecb3f7e-2da6-4071-8151-6b83ced04061</id>
      <masked>false</masked>
      <name>trackingId</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.transactionId</defaultValue>
      <description></description>
      <id>884724f5-17c3-487c-af2b-2ad6f5cfb210</id>
      <masked>false</masked>
      <name>transactionId</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.lineItemId</defaultValue>
      <description></description>
      <id>5b0b5992-78ce-45e2-892d-f0231d59d77e</id>
      <masked>false</masked>
      <name>lineItemId</name>
   </variables>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
