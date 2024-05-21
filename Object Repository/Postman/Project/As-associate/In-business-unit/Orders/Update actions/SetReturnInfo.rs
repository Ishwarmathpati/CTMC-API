<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>SetReturnInfo</name>
   <tag></tag>
   <elementGuidId>06c48928-2682-44f5-b476-6738e7a25625</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>true</autoUpdateContent>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n    \&quot;version\&quot;: ${order-version},\n    \&quot;actions\&quot;: [\n        {\n            \&quot;action\&quot; : \&quot;setReturnInfo\&quot;,\n            \&quot;items\&quot; : [ {\n              \&quot;items\&quot; : [ {\n                \&quot;quantity\&quot; : 1,\n                \&quot;lineItemId\&quot; : \&quot;${lineItemId}\&quot;,\n                \&quot;comment\&quot; : \&quot;any comment\&quot;,\n                \&quot;shipmentState\&quot; : \&quot;Returned\&quot;\n              } ],\n              \&quot;returnTrackingId\&quot; : \&quot;returnTrackingId-example\&quot;,\n              \&quot;returnDate\&quot; : \&quot;2021-04-30T09:21:15.003Z\&quot;\n            } ]\n          }\n    ]\n}&quot;,
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
      <webElementGuid>299efaa5-d522-4452-bf94-20d9cad662c9</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>8a24b002-eb4c-4f84-919b-8c6bd4e346d2</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.6.9</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${host}/${project-key}/as-associate/${associate-id}/in-business-unit/key=${business-unit-key}/orders/${order-id}</restUrl>
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
      <id>0ce53515-bf21-4517-ac02-cef1e8a75a79</id>
      <masked>false</masked>
      <name>host</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.project-key</defaultValue>
      <description></description>
      <id>d8ca7de1-886d-46fb-9b25-ec1dede3c2a0</id>
      <masked>false</masked>
      <name>project-key</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.associate-id</defaultValue>
      <description></description>
      <id>7610ebab-6c0c-4ba3-bfae-d4c571d14b19</id>
      <masked>false</masked>
      <name>associate-id</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.business-unit-key</defaultValue>
      <description></description>
      <id>a718c5f4-7607-431e-b2a0-c0cb399ca403</id>
      <masked>false</masked>
      <name>business-unit-key</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.order-id</defaultValue>
      <description></description>
      <id>49bf3c3d-17d1-4228-9557-e057fd568724</id>
      <masked>false</masked>
      <name>order-id</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.order-version</defaultValue>
      <description></description>
      <id>def6a31a-f6b0-49e7-a661-6b6bee2938e8</id>
      <masked>false</masked>
      <name>order-version</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.lineItemId</defaultValue>
      <description></description>
      <id>89c7d46d-34b1-4935-853c-6a7f9836b718</id>
      <masked>false</masked>
      <name>lineItemId</name>
   </variables>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
