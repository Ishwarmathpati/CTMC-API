<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>SetBillingAddressCustomType</name>
   <tag></tag>
   <elementGuidId>18717294-377e-4a5c-8fa8-98a1e4f9819b</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>true</autoUpdateContent>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n    \&quot;version\&quot;: ${cart-version},\n    \&quot;actions\&quot;: [\n        {\n            \&quot;action\&quot; : \&quot;setBillingAddressCustomType\&quot;,\n            \&quot;type\&quot; : {\n              \&quot;id\&quot; : \&quot;${type-id}\&quot;,\n              \&quot;typeId\&quot; : \&quot;type\&quot;\n            },\n            \&quot;fields\&quot; : {\n              \&quot;exampleStringTypeField\&quot; : \&quot;TextString\&quot;\n            }\n          }\n    ]\n}&quot;,
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
      <webElementGuid>d638d115-1c37-4c10-b1fa-f52771854794</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>c19ab8f9-2d9d-42d2-94f6-4171297d63a0</webElementGuid>
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
      <id>7a45f9df-aa48-4ffb-96f7-bdc488df6114</id>
      <masked>false</masked>
      <name>host</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.project-key</defaultValue>
      <description></description>
      <id>17428293-d4ef-4aba-9b61-5147300e344d</id>
      <masked>false</masked>
      <name>project-key</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.cart-id</defaultValue>
      <description></description>
      <id>eed62e4e-ee43-4d41-bba6-1310a01f7afe</id>
      <masked>false</masked>
      <name>cart-id</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.cart-version</defaultValue>
      <description></description>
      <id>c15fe088-c814-4c02-9914-389a6f3372b4</id>
      <masked>false</masked>
      <name>cart-version</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.type-id</defaultValue>
      <description></description>
      <id>60544a76-ec52-493d-bae8-0e145fcb3ee6</id>
      <masked>false</masked>
      <name>type-id</name>
   </variables>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
