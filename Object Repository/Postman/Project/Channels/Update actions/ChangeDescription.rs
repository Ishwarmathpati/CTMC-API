<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>ChangeDescription</name>
   <tag></tag>
   <elementGuidId>7c711034-efc7-40c5-87a7-b1610963e54b</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>true</autoUpdateContent>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n    \&quot;version\&quot;: ${channel-version},\n    \&quot;actions\&quot;: [\n        {\n            \&quot;action\&quot; : \&quot;changeDescription\&quot;,\n            \&quot;description\&quot; : {\n              \&quot;en\&quot; : \&quot;new Description EN\&quot;,\n              \&quot;de\&quot; : \&quot;new Description DE\&quot;\n            }\n          }\n    ]\n}&quot;,
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
      <webElementGuid>e5942684-5f03-4d05-b888-5fccfd7f6561</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>249541ac-1bf7-4332-810b-7bd2815a2e23</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.6.9</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${host}/${project-key}/channels/${channel-id}</restUrl>
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
      <id>4f02c208-63e5-4020-87ba-1ecdd231525d</id>
      <masked>false</masked>
      <name>host</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.project-key</defaultValue>
      <description></description>
      <id>c4423ca7-72a3-4a8c-a166-da43787dd4eb</id>
      <masked>false</masked>
      <name>project-key</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.channel-id</defaultValue>
      <description></description>
      <id>6ade2da3-53a7-41d7-8908-7cdae3c9e55a</id>
      <masked>false</masked>
      <name>channel-id</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.channel-version</defaultValue>
      <description></description>
      <id>c30ced9a-217d-48fe-bdd4-74db84e109f7</id>
      <masked>false</masked>
      <name>channel-version</name>
   </variables>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
