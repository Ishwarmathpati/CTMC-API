<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>SetAssetSources</name>
   <tag></tag>
   <elementGuidId>4c4f7845-87ac-482c-b1f9-d88e1d4041a3</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>true</autoUpdateContent>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n    \&quot;version\&quot;: ${product-version},\n    \&quot;actions\&quot;: [\n        {\n            \&quot;action\&quot; : \&quot;setAssetSources\&quot;,\n            \&quot;assetId\&quot; : \&quot;${assetId}\&quot;,\n            \&quot;sources\&quot; : [ {\n              \&quot;uri\&quot; : \&quot;https://www.commercetools.de/ct-logo.svg\&quot;,\n              \&quot;key\&quot; : \&quot;vector\&quot;\n            } ]\n          }\n    ]\n}&quot;,
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
      <webElementGuid>5a74d760-fec9-4145-b239-5f5dbb8bfe59</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>ce62085f-44b8-482d-813b-9b5a9f196df5</webElementGuid>
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
      <id>7e44c51b-1e67-4a31-ade5-81101c8d611f</id>
      <masked>false</masked>
      <name>host</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.project-key</defaultValue>
      <description></description>
      <id>c8d169bc-81d3-4dde-831a-3f43a606d26e</id>
      <masked>false</masked>
      <name>project-key</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.product-id</defaultValue>
      <description></description>
      <id>dd174eee-8cb3-45dd-8947-44de0b6f88be</id>
      <masked>false</masked>
      <name>product-id</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.product-version</defaultValue>
      <description></description>
      <id>17d29e07-b5df-47b8-b854-fba1f08c197d</id>
      <masked>false</masked>
      <name>product-version</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.assetId</defaultValue>
      <description></description>
      <id>db86955c-07fd-4401-8b06-b8c34df70c8e</id>
      <masked>false</masked>
      <name>assetId</name>
   </variables>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
