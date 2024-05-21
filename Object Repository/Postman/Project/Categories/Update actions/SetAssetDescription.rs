<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>SetAssetDescription</name>
   <tag></tag>
   <elementGuidId>924a2484-c9aa-4b16-8761-083b162a37e5</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>true</autoUpdateContent>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n    \&quot;version\&quot;: ${category-version},\n    \&quot;actions\&quot;: [\n        {\n            \&quot;action\&quot; : \&quot;setAssetDescription\&quot;,\n            \&quot;assetId\&quot; : \&quot;${assetId}\&quot;,\n            \&quot;description\&quot; : {\n              \&quot;de\&quot; : \&quot;Dies ist eine Asset-Beschreibung\&quot;,\n              \&quot;en\&quot; : \&quot;This is an asset description\&quot;\n            }\n          }\n    ]\n}&quot;,
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
      <webElementGuid>3d2b67a2-c6d4-4f51-b320-d87f99f7919b</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>b520473c-d8ee-413d-ba76-a1d6d0680fae</webElementGuid>
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
      <id>6567bb3c-4032-4030-9b21-ea421e8e0a8f</id>
      <masked>false</masked>
      <name>host</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.project-key</defaultValue>
      <description></description>
      <id>56650753-5cfb-42ac-818c-615dc36aa954</id>
      <masked>false</masked>
      <name>project-key</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.category-id</defaultValue>
      <description></description>
      <id>746adb64-c7bb-45be-8734-a1d8adfd84ee</id>
      <masked>false</masked>
      <name>category-id</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.category-version</defaultValue>
      <description></description>
      <id>f8da0a17-2aa8-4511-8053-3b2520f7053b</id>
      <masked>false</masked>
      <name>category-version</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.assetId</defaultValue>
      <description></description>
      <id>9e0df406-5bae-4b95-9ec3-048b9c360c3c</id>
      <masked>false</masked>
      <name>assetId</name>
   </variables>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
