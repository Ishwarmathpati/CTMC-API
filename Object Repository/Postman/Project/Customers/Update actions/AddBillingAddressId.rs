<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>AddBillingAddressId</name>
   <tag></tag>
   <elementGuidId>9de0577c-52b2-45da-a3e5-030ca482de49</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>true</autoUpdateContent>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n    \&quot;version\&quot;: ${customer-version},\n    \&quot;actions\&quot;: [\n        {\n            \&quot;action\&quot; : \&quot;addBillingAddressId\&quot;,\n            \&quot;addressId\&quot; : \&quot;${addressId}\&quot;\n          }\n    ]\n}&quot;,
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
      <webElementGuid>6b9c8f24-cd0f-4b32-af8b-50fda7710253</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>1cab9622-6697-4952-b573-79691f8a3beb</webElementGuid>
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
      <id>18a6e250-0f09-4273-8a30-e9e1d9dc9a88</id>
      <masked>false</masked>
      <name>host</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.project-key</defaultValue>
      <description></description>
      <id>8b4336af-7cbf-4ee7-9d6a-28fe7705750b</id>
      <masked>false</masked>
      <name>project-key</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.customer-id</defaultValue>
      <description></description>
      <id>7d6a3a2c-a43a-44e1-97b8-2b5f76d972b2</id>
      <masked>false</masked>
      <name>customer-id</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.customer-version</defaultValue>
      <description></description>
      <id>46735fe7-011b-4f73-bbd1-7e56b4ac1c48</id>
      <masked>false</masked>
      <name>customer-version</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.addressId</defaultValue>
      <description></description>
      <id>162aae89-c4f7-4d4e-b9d2-2d29c699b554</id>
      <masked>false</masked>
      <name>addressId</name>
   </variables>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
