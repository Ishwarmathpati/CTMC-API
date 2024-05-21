<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>RemoveLineItem</name>
   <tag></tag>
   <elementGuidId>cfc599ea-c0bd-4ee5-b4de-49150d52e923</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>true</autoUpdateContent>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n    \&quot;version\&quot;: ${cart-version},\n    \&quot;actions\&quot;: [\n        {\n            \&quot;action\&quot; : \&quot;removeLineItem\&quot;,\n            \&quot;lineItemId\&quot; : \&quot;${lineItemId}\&quot;,\n            \&quot;quantity\&quot; : 1,\n            \&quot;externalPrice\&quot; : {\n              \&quot;currencyCode\&quot; : \&quot;EUR\&quot;,\n              \&quot;centAmount\&quot; : 4000\n            },\n            \&quot;shippingDetailsToRemove\&quot; : {\n              \&quot;targets\&quot; : [ {\n                \&quot;addressKey\&quot; : \&quot;AddressKeyStringFromAddress\&quot;,\n                \&quot;quantity\&quot; : 2\n              } ]\n            }\n          }\n    ]\n}&quot;,
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
      <webElementGuid>92e0864d-3688-4c5d-bd1e-79de5385bd9d</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>74f62e58-b29d-49ee-bfe5-e5b630fd9d02</webElementGuid>
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
      <id>83faca5e-38b0-47c3-8b49-b098270efa36</id>
      <masked>false</masked>
      <name>host</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.project-key</defaultValue>
      <description></description>
      <id>38b11464-6c9a-4050-999c-889e0efb10c7</id>
      <masked>false</masked>
      <name>project-key</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.cart-id</defaultValue>
      <description></description>
      <id>e375dadf-2ec6-4fd2-8e21-9d4f2e3322f8</id>
      <masked>false</masked>
      <name>cart-id</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.cart-version</defaultValue>
      <description></description>
      <id>72ca3627-e20b-42cd-b65e-e4171ea6a833</id>
      <masked>false</masked>
      <name>cart-version</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.lineItemId</defaultValue>
      <description></description>
      <id>06dcbb7d-ac8d-4777-98a8-74761eb1b3de</id>
      <masked>false</masked>
      <name>lineItemId</name>
   </variables>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
