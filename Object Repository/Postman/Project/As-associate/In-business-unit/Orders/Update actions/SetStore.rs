<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>SetStore</name>
   <tag></tag>
   <elementGuidId>754ddfae-67ea-4b53-bc18-39158fa76ece</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>true</autoUpdateContent>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n    \&quot;version\&quot;: ${order-version},\n    \&quot;actions\&quot;: [\n        {\n            \&quot;action\&quot; : \&quot;setStore\&quot;,\n            \&quot;store\&quot; : {\n              \&quot;key\&quot; : \&quot;${store-key}\&quot;,\n              \&quot;typeId\&quot; : \&quot;store\&quot;\n            }\n          }\n    ]\n}&quot;,
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
      <webElementGuid>18c058e7-d77c-4e37-8f9f-08e625b44e3f</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>2a599e45-13fd-495e-a11a-2c3ad63b69bd</webElementGuid>
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
      <id>a0cd13c3-029f-42f3-a9bc-a39e0c9c4e8d</id>
      <masked>false</masked>
      <name>host</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.project-key</defaultValue>
      <description></description>
      <id>aa2fa21a-8f5c-4d06-b6b2-84479d6e9848</id>
      <masked>false</masked>
      <name>project-key</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.associate-id</defaultValue>
      <description></description>
      <id>a1651aae-13c7-4b3f-bee9-eda935aefc18</id>
      <masked>false</masked>
      <name>associate-id</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.business-unit-key</defaultValue>
      <description></description>
      <id>9d6c254e-4187-430d-af7b-e66d642731b1</id>
      <masked>false</masked>
      <name>business-unit-key</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.order-id</defaultValue>
      <description></description>
      <id>77fbf0d6-df78-4926-b0e7-bb078617ce03</id>
      <masked>false</masked>
      <name>order-id</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.order-version</defaultValue>
      <description></description>
      <id>403d7734-f1e5-4021-bf8e-010beee16c94</id>
      <masked>false</masked>
      <name>order-version</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.store-key</defaultValue>
      <description></description>
      <id>493e3dd7-755e-4eed-ae60-693d9f1c5636</id>
      <masked>false</masked>
      <name>store-key</name>
   </variables>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
