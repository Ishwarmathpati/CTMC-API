<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>SetAssetKey</name>
   <tag></tag>
   <elementGuidId>9ddbd4af-930d-494c-8660-9298dd5bd0e5</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>true</autoUpdateContent>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n    \&quot;version\&quot;: ${product-version},\n    \&quot;actions\&quot;: [\n        {\n            \&quot;action\&quot; : \&quot;setAssetKey\&quot;,\n            \&quot;assetId\&quot; : \&quot;${assetId}\&quot;,\n            \&quot;assetKey\&quot; : \&quot;assetKeyString\&quot;\n          }\n    ]\n}&quot;,
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
      <webElementGuid>405a8977-69d9-4004-8adc-0a6abf1e6184</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>34e670c2-d4c9-44da-81ca-a13018ed1991</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.6.9</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${host}/${project-key}/products/${product-id}</restUrl>
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
      <id>766beac4-102b-40d0-b942-186f4fa32ebd</id>
      <masked>false</masked>
      <name>host</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.project-key</defaultValue>
      <description></description>
      <id>e1dba62d-b439-438c-b569-bc74a37ab41e</id>
      <masked>false</masked>
      <name>project-key</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.product-id</defaultValue>
      <description></description>
      <id>653bd25b-0673-4ee4-8464-1e5f51b21be9</id>
      <masked>false</masked>
      <name>product-id</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.product-version</defaultValue>
      <description></description>
      <id>f9b08ca7-9391-4741-bc24-87cbbf139724</id>
      <masked>false</masked>
      <name>product-version</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.assetId</defaultValue>
      <description></description>
      <id>82fef507-01a4-498d-b152-c1dce9527bd5</id>
      <masked>false</masked>
      <name>assetId</name>
   </variables>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
