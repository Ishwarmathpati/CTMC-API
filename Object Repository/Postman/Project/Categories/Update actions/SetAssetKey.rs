<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>SetAssetKey</name>
   <tag></tag>
   <elementGuidId>64a3728d-0c80-45cb-896d-7707279ba171</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>true</autoUpdateContent>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n    \&quot;version\&quot;: ${category-version},\n    \&quot;actions\&quot;: [\n        {\n            \&quot;action\&quot; : \&quot;setAssetKey\&quot;,\n            \&quot;assetId\&quot; : \&quot;${assetId}\&quot;\n          }\n    ]\n}&quot;,
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
      <webElementGuid>a110dbad-5853-485d-844a-6f8a751fecee</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>5bf922be-37c3-497c-b379-5bf6fda2d536</webElementGuid>
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
      <id>8f4608f9-0698-4cf3-82c2-ce02735988ba</id>
      <masked>false</masked>
      <name>host</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.project-key</defaultValue>
      <description></description>
      <id>65394125-768a-4add-a17f-10fa63130863</id>
      <masked>false</masked>
      <name>project-key</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.category-id</defaultValue>
      <description></description>
      <id>3f7e8559-ebf9-4306-84f9-636d85b1a0e4</id>
      <masked>false</masked>
      <name>category-id</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.category-version</defaultValue>
      <description></description>
      <id>818bf77f-1627-4232-9e7f-f6df94102ee0</id>
      <masked>false</masked>
      <name>category-version</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.assetId</defaultValue>
      <description></description>
      <id>cd55de29-5da0-4528-899c-5145812742c6</id>
      <masked>false</masked>
      <name>assetId</name>
   </variables>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
