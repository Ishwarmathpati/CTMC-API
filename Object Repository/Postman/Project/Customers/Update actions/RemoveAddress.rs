<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>RemoveAddress</name>
   <tag></tag>
   <elementGuidId>a2eee622-b2ec-43a1-88f4-3bf6fb095368</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>true</autoUpdateContent>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n    \&quot;version\&quot;: ${customer-version},\n    \&quot;actions\&quot;: [\n        {\n            \&quot;action\&quot; : \&quot;removeAddress\&quot;,\n            \&quot;addressId\&quot; : \&quot;${addressId}\&quot;\n          }\n    ]\n}&quot;,
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
      <webElementGuid>466785dd-296f-4c35-b300-6e027e2f6fef</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>adebb22b-861f-40b7-b839-d2fa32d5f9fb</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.6.9</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${host}/${project-key}/customers/${customer-id}</restUrl>
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
      <id>0fc90dbd-d97f-429f-ac90-4c530c5ffcdd</id>
      <masked>false</masked>
      <name>host</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.project-key</defaultValue>
      <description></description>
      <id>8da5c8e4-1b21-46bd-a378-a69080f61108</id>
      <masked>false</masked>
      <name>project-key</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.customer-id</defaultValue>
      <description></description>
      <id>0f976d0d-45cc-46d4-9735-4ea33f8ba578</id>
      <masked>false</masked>
      <name>customer-id</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.customer-version</defaultValue>
      <description></description>
      <id>1dd52963-74f7-4eb2-938e-49528950d15d</id>
      <masked>false</masked>
      <name>customer-version</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.addressId</defaultValue>
      <description></description>
      <id>c36c5ef3-ab18-4bbc-b1ae-9bf072ddf82b</id>
      <masked>false</masked>
      <name>addressId</name>
   </variables>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
