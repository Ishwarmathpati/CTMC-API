<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>ChangeAssetOrder</name>
   <tag></tag>
   <elementGuidId>3510d026-a309-4256-8250-809ad3225e83</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>true</autoUpdateContent>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n    \&quot;version\&quot;: ${category-version},\n    \&quot;actions\&quot;: [\n        {\n            \&quot;action\&quot; : \&quot;changeAssetOrder\&quot;,\n            \&quot;assetOrder\&quot; : [ \&quot;${assetId1}\&quot;, \&quot;${assetId2}\&quot; ]\n          }\n    ]\n}&quot;,
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
      <webElementGuid>332aa80d-6e07-46ac-94c5-c53c04711953</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>2a23b0a2-4971-4a01-9dce-4a6bd9aea727</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.6.9</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${host}/${project-key}/categories/${category-id}</restUrl>
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
      <id>4e892b78-1899-43bc-af82-6e14ac693b9d</id>
      <masked>false</masked>
      <name>host</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.project-key</defaultValue>
      <description></description>
      <id>44f34d5e-75a8-4eb5-a453-e2e59dc33b7d</id>
      <masked>false</masked>
      <name>project-key</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.category-id</defaultValue>
      <description></description>
      <id>44d9bc1e-f347-455c-b76e-92fa5de51a2a</id>
      <masked>false</masked>
      <name>category-id</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.category-version</defaultValue>
      <description></description>
      <id>d3f4164d-08c4-405b-8f53-08c422c7e696</id>
      <masked>false</masked>
      <name>category-version</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.assetId1</defaultValue>
      <description></description>
      <id>0164fd21-2871-4d2f-ba51-30ab40385c57</id>
      <masked>false</masked>
      <name>assetId1</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.assetId2</defaultValue>
      <description></description>
      <id>5425d3e7-75c5-4a40-81a3-ad52c3c06a98</id>
      <masked>false</masked>
      <name>assetId2</name>
   </variables>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
