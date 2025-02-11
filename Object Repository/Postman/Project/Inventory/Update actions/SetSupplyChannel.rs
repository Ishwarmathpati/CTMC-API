<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>SetSupplyChannel</name>
   <tag></tag>
   <elementGuidId>43cdf9a1-9e86-44d7-85ed-447e182fdf4c</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>true</autoUpdateContent>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n    \&quot;version\&quot;: ${inventory-version},\n    \&quot;actions\&quot;: [\n        {\n            \&quot;action\&quot; : \&quot;setSupplyChannel\&quot;,\n            \&quot;supplyChannel\&quot; : {\n              \&quot;id\&quot; : \&quot;${supplyChannelId}\&quot;,\n              \&quot;typeId\&quot; : \&quot;channel\&quot;\n            }\n          }\n    ]\n}&quot;,
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
      <webElementGuid>138bcb1e-2cab-4f00-bdae-baa92aa2c297</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>427fe9aa-670a-49be-ae8c-ece449e63ef5</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.6.9</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${host}/${project-key}/inventory/${inventory-id}</restUrl>
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
      <id>346acd0f-d192-4ae4-952e-09a54a97bb78</id>
      <masked>false</masked>
      <name>host</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.project-key</defaultValue>
      <description></description>
      <id>2a4f7f74-5577-42d2-930d-bf096949e8fb</id>
      <masked>false</masked>
      <name>project-key</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.inventory-id</defaultValue>
      <description></description>
      <id>1c78ba2c-2efd-4d2e-b71e-9f36e2ec16a7</id>
      <masked>false</masked>
      <name>inventory-id</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.inventory-version</defaultValue>
      <description></description>
      <id>cfec99fc-0ceb-453a-b6a2-db05e66d7cf6</id>
      <masked>false</masked>
      <name>inventory-version</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.supplyChannelId</defaultValue>
      <description></description>
      <id>a473a9b1-5a43-4a6d-b08c-894af7e9e944</id>
      <masked>false</masked>
      <name>supplyChannelId</name>
   </variables>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
