<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>SetCustomType</name>
   <tag></tag>
   <elementGuidId>746aad77-1346-4d69-bfe0-71718d4b47b1</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>true</autoUpdateContent>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n    \&quot;version\&quot;: ${store-version},\n    \&quot;actions\&quot;: [\n        {\n            \&quot;action\&quot; : \&quot;setCustomType\&quot;,\n            \&quot;type\&quot; : {\n              \&quot;id\&quot; : \&quot;${type-id}\&quot;,\n              \&quot;typeId\&quot; : \&quot;type\&quot;\n            },\n            \&quot;fields\&quot; : {\n              \&quot;exampleStringTypeField\&quot; : \&quot;TextString\&quot;\n            }\n          }\n    ]\n}&quot;,
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
      <webElementGuid>3ca7202c-e812-480a-926e-88638dc7794d</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>5ad1ad14-0661-45de-a841-e478ee2ca58a</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.6.9</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${host}/${project-key}/stores/${store-id}</restUrl>
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
      <id>4a2a7081-7fee-4094-b42d-c87d2feda046</id>
      <masked>false</masked>
      <name>host</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.project-key</defaultValue>
      <description></description>
      <id>d2ea35d1-3d74-4972-ab64-8f80e700b5b4</id>
      <masked>false</masked>
      <name>project-key</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.store-id</defaultValue>
      <description></description>
      <id>c1358942-6b19-4fd9-88b9-1b60f313521c</id>
      <masked>false</masked>
      <name>store-id</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.store-version</defaultValue>
      <description></description>
      <id>c0b57a92-6ae8-4c0c-b166-735eee499288</id>
      <masked>false</masked>
      <name>store-version</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.type-id</defaultValue>
      <description></description>
      <id>feb5200a-dac0-4019-89bd-950638285e91</id>
      <masked>false</masked>
      <name>type-id</name>
   </variables>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
