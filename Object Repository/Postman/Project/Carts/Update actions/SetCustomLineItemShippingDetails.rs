<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>SetCustomLineItemShippingDetails</name>
   <tag></tag>
   <elementGuidId>c7736bb3-96c1-4e10-a869-6076d03005e2</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>true</autoUpdateContent>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n    \&quot;version\&quot;: ${cart-version},\n    \&quot;actions\&quot;: [\n        {\n            \&quot;action\&quot; : \&quot;setCustomLineItemShippingDetails\&quot;,\n            \&quot;customLineItemId\&quot; : \&quot;${customlineItemId}\&quot;,\n            \&quot;shippingDetails\&quot; : {\n              \&quot;targets\&quot; : [ {\n                \&quot;addressKey\&quot; : \&quot;${addressKey}\&quot;,\n                \&quot;quantity\&quot; : 1\n              } ]\n            }\n          }\n    ]\n}&quot;,
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
      <webElementGuid>a758c164-2d0f-46ef-9978-904a353fb53d</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>0838da83-a4cd-48f0-a5ea-3d4391a22f22</webElementGuid>
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
      <id>ba9d75a0-2a2b-48d5-bd15-517de96c6cf6</id>
      <masked>false</masked>
      <name>host</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.project-key</defaultValue>
      <description></description>
      <id>d4c2a66a-6468-4c05-8cc5-7213fadeb6cb</id>
      <masked>false</masked>
      <name>project-key</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.cart-id</defaultValue>
      <description></description>
      <id>47aeb449-ce3a-472a-8683-0bd022461316</id>
      <masked>false</masked>
      <name>cart-id</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.cart-version</defaultValue>
      <description></description>
      <id>45b6b39d-025d-4faa-bb39-04b87eaf0611</id>
      <masked>false</masked>
      <name>cart-version</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.customlineItemId</defaultValue>
      <description></description>
      <id>7c4fc5d1-1b7c-4f39-97fe-b605014672c5</id>
      <masked>false</masked>
      <name>customlineItemId</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.addressKey</defaultValue>
      <description></description>
      <id>376cf4eb-c96f-4c02-b4bb-f95ee4ab1f51</id>
      <masked>false</masked>
      <name>addressKey</name>
   </variables>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
